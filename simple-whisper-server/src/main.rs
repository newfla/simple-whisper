use std::str::FromStr;

use axum::{
    extract::{ws::WebSocket, Path, Query, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    serve, Json, Router,
};
use serde::{Deserialize, Serialize};
use simple_whisper::{Language, Model};
use strum::{EnumIs, IntoEnumIterator, ParseError};
use thiserror::Error;
use tokio::net::TcpListener;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid ID")]
    InvalidID(#[from] ParseError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::InvalidID(_) => (StatusCode::NOT_FOUND, format!("{self}")),
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
    ignore_cache: bool
}

#[derive(EnumIs, Deserialize, Serialize)]
enum ModelDownloadResponse {
    Started,
    Completed,
    Failed,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .nest("/languages", languages())
        .nest("/models", models())
}

fn languages() -> Router {
    Router::new()
        .route("/list", get(list_languages))
        .route("/check/:id", get(valid_language))
}

async fn list_languages() -> Json<Vec<LanguageResponse>> {
    Json(
        Language::iter()
            .map(|l| {
                let binding = l.to_string();
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
    Language::from_str(&id).map(|_| ()).map_err(Into::into)
}

fn models() -> Router {
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

async fn download_model(ws: WebSocketUpgrade, Path(id): Path<String>, parameters: Query<ModelParameters>) -> Response {
    let maybe_model: Result<Model, Error> = Model::from_str(&id).map_err(Into::into);
    match maybe_model {
        Ok(model) => ws.on_upgrade(|socket| handle_download_model(socket, model, parameters.0)),
        Err(err) => err.into_response(),
    }
}
async fn handle_download_model(socket: WebSocket, model: Model, params: ModelParameters) {
    let _ = internal_handle_download_model(socket, model, params).await;
}

async fn internal_handle_download_model(mut socket: WebSocket, model: Model, params: ModelParameters) -> anyhow::Result<()> {
    socket
        .send(axum::extract::ws::Message::Text(serde_json::to_string(
            &ModelDownloadResponse::Started,
        )?))
        .await?;
    match model.download_model(false, params.ignore_cache).await {
        Ok(_) => {
            socket
                .send(axum::extract::ws::Message::Text(serde_json::to_string(
                    &ModelDownloadResponse::Completed,
                )?))
                .await?
        }
        Err(_) => {
            socket
                .send(axum::extract::ws::Message::Text(serde_json::to_string(
                    &ModelDownloadResponse::Failed,
                )?))
                .await?
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::future::IntoFuture;

    use axum::serve;
    use futures::StreamExt;
    use reqwest::Client;
    use reqwest_websocket::{Message, RequestBuilderExt};
    use tokio::{net::TcpListener, spawn};

    use crate::{app, LanguageResponse, ModelDownloadResponse, ModelResponse};

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
        assert_eq!(bad_request.as_u16(), 404);
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
        assert_eq!(10, models.len());

        let websocket = Client::default()
            .get("ws://127.0.0.1:4000/models/download/tiny_en?ignore_cache=false")
            .upgrade()
            .send()
            .await
            .unwrap()
            .into_websocket()
            .await
            .unwrap();

        let (_, mut rx) = websocket.split();
        while let Some(Ok(Message::Text(msg))) = rx.next().await {
            let msg: ModelDownloadResponse = serde_json::from_str(&msg).unwrap();
            assert!(msg.is_started() || msg.is_completed())
        }
    }
}
