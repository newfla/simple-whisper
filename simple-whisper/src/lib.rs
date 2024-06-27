use std::{path::Path, sync::Arc, time::Duration};

use derive_builder::Builder;

mod lang;
mod model;

pub use lang::Language;
pub use model::Model;
use strum::{Display, EnumIs};
use thiserror::Error;
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver},
        Notify,
    },
};

#[derive(Default, Builder, Debug)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct Whisper {
    language: Language,
    model: Model,
    #[builder(default = "false")]
    progress_bar: bool,
    #[builder(default = "false")]
    force_download: bool,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DownloadFail(#[from] hf_hub::api::tokio::ApiError),
}

#[derive(Clone, Debug, Display, EnumIs)]
pub enum Event {
    #[strum(to_string = "Downloading {file}")]
    DownloadStarted { file: String },
    #[strum(to_string = "{file} has been downloaded")]
    DownloadCompleted { file: String },
    #[strum(to_string = "{transcription}")]
    Segment {
        start_offset: f32,
        end_offset: f32,
        percentage: f32,
        transcription: String,
    },
}

impl WhisperBuilder {
    fn validate(&self) -> Result<(), WhisperBuilderError> {
        if self.language.as_ref().is_some_and(|l| !l.is_english())
            && self.model.as_ref().is_some_and(|m| !m.is_multilang())
        {
            let err = format!(
                "The requested language {} is not compatible with {} model",
                self.language.as_ref().unwrap(),
                self.model.as_ref().unwrap()
            );
            return Err(WhisperBuilderError::ValidationError(err));
        }
        Ok(())
    }
}

impl Whisper {
    pub fn transcribe(self, file: impl AsRef<Path>) -> UnboundedReceiver<Result<Event, Error>> {
        let (tx, rx) = unbounded_channel();
        let (tx_event, mut rx_event) = unbounded_channel();

        let notify = Arc::new(Notify::new());
        let notify2 = notify.clone();

        // Download events forwarder
        let tx_forwarder = tx.clone();
        spawn(async move {
            while let Some(msg) = rx_event.recv().await {
                let _ = tx_forwarder.send(Ok(msg));
            }
            notify.notify_one();
        });

        spawn(async move {
            let model = self
                .model
                .download_model_listener(self.progress_bar, self.force_download, tx_event)
                .await;
            notify2.notified().await;
            match model {
                Ok(model) => {
                    //Stub send
                    let _ = tx.send(Ok(Event::Segment {
                        start_offset: 0.,
                        end_offset: 0.,
                        percentage: 0.,
                        transcription: "Stub".to_owned(),
                    }));

                    tokio::time::sleep(Duration::from_secs(2)).await;
                    let _ = tx.send(Ok(Event::Segment {
                        start_offset: 0.,
                        end_offset: 0.,
                        percentage: 0.5,
                        transcription: "Stub".to_owned(),
                    }));
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    let _ = tx.send(Ok(Event::Segment {
                        start_offset: 0.,
                        end_offset: 0.,
                        percentage: 1.,
                        transcription: "Stub".to_owned(),
                    }));
                }
                Err(err) => {
                    let _ = tx.send(Err(err));
                }
            }
        });

        rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_file {
        ($file_name:expr) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/", $file_name)
        };
    }

    #[test]
    fn incompatible_lang_model() {
        let error = WhisperBuilder::default()
            .language(Language::Italian)
            .model(Model::BaseEn)
            .build()
            .unwrap_err();
        assert!(matches!(error, WhisperBuilderError::ValidationError(_)));
    }

    #[test]
    fn compatible_lang_model() {
        WhisperBuilder::default()
            .language(Language::Italian)
            .model(Model::Base)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn simple_transcribe_ok() {
        let mut rx = WhisperBuilder::default()
            .language(Language::Italian)
            .model(Model::Tiny)
            .progress_bar(true)
            .build()
            .unwrap()
            .transcribe(test_file!("samples_jfk.mp3"));

        while let Some(msg) = rx.recv().await {
            assert!(msg.is_ok());
            println!("{msg:?}");
        }
    }
}
