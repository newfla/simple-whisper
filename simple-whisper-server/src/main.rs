use std::{str::FromStr, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        DefaultBodyLimit, MatchedPath, Path, Query, WebSocketUpgrade,
    },
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    serve, Json, Router,
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use simple_whisper::{Event, Language, Model, Whisper, WhisperBuilder};
use strum::{EnumIs, EnumMessage, IntoEnumIterator};
use tempfile::NamedTempFile;
use thiserror::Error;
use tokio::{fs::write, net::TcpListener, spawn, sync::mpsc::unbounded_channel};
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Server listening port
    #[arg(long, short = 'p', default_value = "3000")]
    server_port: u16,
}

#[derive(Error, Debug)]
enum Error {
    #[error("Model {0} not supported")]
    ModelNotSupported(String),
    #[error("Language {0} not supported")]
    LanguageNotSupported(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::ModelNotSupported(_) => (StatusCode::BAD_REQUEST, format!("{self}")),
            Error::LanguageNotSupported(_) => (StatusCode::BAD_REQUEST, format!("{self}")),
        }
        .into_response()
    }
}
#[derive(Deserialize, Serialize)]
struct LanguageResponse {
    id: String,
    lang: String,
}

#[derive(Deserialize, Serialize)]
struct ModelResponse {
    id: String,
    model: String,
}

#[derive(Deserialize)]
struct ModelParameters {
    ignore_cache: bool,
}

#[derive(Deserialize)]
struct TranscribeParameters {
    #[serde(default)]
    ignore_cache: bool,
}

#[derive(EnumIs, Debug, Deserialize, Serialize)]
enum ServerResponse {
    FileStarted {
        file: String,
    },
    FileCompleted {
        file: String,
    },
    FileProgress {
        file: String,
        percentage: f32,
        elapsed_time: Duration,
        remaining_time: Duration,
    },
    Failed,
    DownloadModelCompleted,
    Segment {
        start_offset: Duration,
        end_offset: Duration,
        percentage: f32,
        transcription: String,
    },
}

impl From<Event> for ServerResponse {
    fn from(value: Event) -> Self {
        match value {
            Event::DownloadStarted { file } => Self::FileStarted { file },
            Event::DownloadCompleted { file } => Self::FileCompleted { file },
            Event::Segment {
                start_offset,
                end_offset,
                percentage,
                transcription,
            } => Self::Segment {
                start_offset,
                end_offset,
                percentage,
                transcription,
            },
            Event::DownloadProgress {
                file,
                percentage,
                elapsed_time,
                remaining_time,
            } => Self::FileProgress {
                file,
                percentage,
                elapsed_time,
                remaining_time,
            },
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "simple-whisper-server=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = TcpListener::bind(("127.0.0.1", cli.server_port))
        .await
        .unwrap();
    serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path
                )
            }),
        )
        .nest("/languages", languages_router())
        .nest("/models", models_router())
        .nest("/transcribe", transcribe_router())
}

fn languages_router() -> Router {
    Router::new()
        .route("/list", get(list_languages))
        .route("/check/:id", get(valid_language))
}

async fn list_languages() -> Json<Vec<LanguageResponse>> {
    Json(
        Language::iter()
            .map(|l| {
                let binding = l.get_message().unwrap();
                let (lang, code) = binding.split_once('-').unwrap();
                LanguageResponse {
                    id: code.trim().to_owned(),
                    lang: lang.trim().to_owned(),
                }
            })
            .collect(),
    )
}

async fn valid_language(Path(id): Path<String>) -> Result<(), Error> {
    Language::from_str(&id)
        .map(|_| ())
        .map_err(|_| Error::LanguageNotSupported(id))
}

fn models_router() -> Router {
    Router::new()
        .route("/list", get(list_models))
        .route("/download/:id", get(download_model))
}

async fn list_models() -> Json<Vec<ModelResponse>> {
    Json(
        Model::iter()
            .map(|l| {
                let binding = l.to_string();
                let (model, code) = binding.split_once('-').unwrap();
                ModelResponse {
                    id: code.trim().to_owned(),
                    model: model.trim().to_owned(),
                }
            })
            .collect(),
    )
}

async fn download_model(
    ws: WebSocketUpgrade,
    Path(id): Path<String>,
    parameters: Query<ModelParameters>,
) -> Response {
    let maybe_model: Result<Model, Error> =
        Model::from_str(&id).map_err(|_| Error::ModelNotSupported(id));
    match maybe_model {
        Ok(model) => ws.on_upgrade(|socket| handle_download_model(socket, model, parameters.0)),
        Err(err) => err.into_response(),
    }
}

async fn handle_download_model(socket: WebSocket, model: Model, params: ModelParameters) {
    let _ = internal_handle_download_model(socket, model, params).await;
}

async fn internal_handle_download_model(
    mut socket: WebSocket,
    model: Model,
    params: ModelParameters,
) -> anyhow::Result<()> {
    let (tx, mut rx) = unbounded_channel();
    let download = spawn(async move {
        model
            .download_model_listener(false, params.ignore_cache, tx)
            .await
    });

    while let Some(msg) = rx.recv().await {
        socket
            .send(Message::Text(serde_json::to_string(
                &Into::<ServerResponse>::into(msg),
            )?))
            .await?;
    }
    match download.await {
        Ok(_) => {
            socket
                .send(Message::Text(serde_json::to_string(
                    &ServerResponse::DownloadModelCompleted,
                )?))
                .await?
        }
        Err(_) => {
            socket
                .send(Message::Text(serde_json::to_string(
                    &ServerResponse::Failed,
                )?))
                .await?
        }
    }
    Ok(())
}

fn transcribe_router() -> Router {
    Router::new()
        .route("/:model/:lang", get(transcribe))
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024))
}

async fn transcribe(
    ws: WebSocketUpgrade,
    Path((model, lang)): Path<(String, String)>,
    parameters: Query<TranscribeParameters>,
) -> Response {
    let model = Model::from_str(&model).map_err(|_| Error::ModelNotSupported(model));
    let lang = Language::from_str(&lang).map_err(|_| Error::LanguageNotSupported(lang));
    if let Err(err) = model {
        return err.into_response();
    }

    if let Err(err) = lang {
        return err.into_response();
    }

    let whisper = WhisperBuilder::default()
        .language(lang.unwrap())
        .model(model.unwrap())
        .force_download(parameters.0.ignore_cache)
        .build()
        .unwrap();

    ws.on_upgrade(|socket| handle_transcription_model(socket, whisper))
}

async fn handle_transcription_model(socket: WebSocket, model: Whisper) {
    let _ = internal_handle_transcription_model(socket, model).await;
}

async fn internal_handle_transcription_model(
    mut socket: WebSocket,
    model: Whisper,
) -> anyhow::Result<()> {
    if let Some(Ok(Message::Binary(data))) = socket.recv().await {
        let file = NamedTempFile::new()?;
        write(file.path(), data).await?;
        let mut rx = model.transcribe(file.path());
        while let Some(msg) = rx.recv().await {
            match msg {
                Ok(msg) => {
                    if msg.is_segment() {
                        socket
                            .send(Message::Text(serde_json::to_string(
                                &Into::<ServerResponse>::into(msg),
                            )?))
                            .await?;
                    }
                }
                Err(_) => {
                    socket
                        .send(Message::Text(serde_json::to_string(
                            &ServerResponse::Failed,
                        )?))
                        .await?
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::future::IntoFuture;

    use axum::serve;
    use futures::{SinkExt, StreamExt};
    use reqwest::Client;
    use reqwest_websocket::{Message, RequestBuilderExt};
    use tokio::{net::TcpListener, spawn};

    use crate::{app, LanguageResponse, ModelResponse, ServerResponse};

    macro_rules! test_file {
        ($file_name:expr) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/", $file_name)
        };
    }

    #[tokio::test]
    async fn integration_test_languages() {
        let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
        spawn(serve(listener, app()).into_future());

        let languages: Vec<LanguageResponse> = reqwest::get("http://127.0.0.1:3000/languages/list")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(99, languages.len());

        let good_request = reqwest::get("http://127.0.0.1:3000/languages/check/en")
            .await
            .unwrap()
            .status();
        assert!(good_request.is_success());

        let bad_request = reqwest::get("http://127.0.0.1:3000/languages/check/zy")
            .await
            .unwrap()
            .status();
        assert_eq!(bad_request.as_u16(), 400);
    }

    #[tokio::test]
    async fn integration_test_models() {
        let listener = TcpListener::bind("127.0.0.1:4000").await.unwrap();
        spawn(serve(listener, app()).into_future());

        let models: Vec<ModelResponse> = reqwest::get("http://127.0.0.1:4000/models/list")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(33, models.len());

        let websocket = Client::default()
            .get("ws://127.0.0.1:4000/models/download/tiny_en?ignore_cache=true")
            .upgrade()
            .send()
            .await
            .unwrap()
            .into_websocket()
            .await
            .unwrap();

        let (_, mut rx) = websocket.split();
        while let Some(Ok(Message::Text(msg))) = rx.next().await {
            let msg: ServerResponse = serde_json::from_str(&msg).unwrap();
            println!("{msg:?}");
            assert!(
                msg.is_file_started()
                    || msg.is_file_completed()
                    || msg.is_file_progress()
                    || msg.is_download_model_completed()
            )
        }
    }

    #[ignore]
    #[tokio::test]
    async fn integration_test_transcription() {
        let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();
        spawn(serve(listener, app()).into_future());

        let client = Client::new();
        let websocket = client
            .get("ws://127.0.0.1:5000/transcribe/tiny/en")
            .upgrade()
            .send()
            .await
            .unwrap()
            .into_websocket()
            .await
            .unwrap();

        let (mut tx, mut rx) = websocket.split();

        let data = tokio::fs::read(test_file!("samples_jfk.wav"))
            .await
            .unwrap();
        tx.send(Message::Binary(data)).await.unwrap();

        while let Some(Ok(Message::Text(msg))) = rx.next().await {
            let msg: ServerResponse = serde_json::from_str(&msg).unwrap();
            println!("{msg:?}");
        }
    }
}
