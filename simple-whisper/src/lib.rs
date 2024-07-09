use std::{
    fs::File,
    io::{self, BufReader},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use burn::backend::{NdArray, Wgpu};
use derive_builder::Builder;
use transcribe::{TranscribeBuilder, TranscribeBuilderError};

mod burn_impl;
mod languages;
mod models;
mod transcribe;

pub use languages::Language;
pub use models::Model;
use rodio::{source::UniformSourceIterator, Decoder, Source};
use strum::{Display, EnumIs};
use thiserror::Error;
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver},
        Notify,
    },
    task::spawn_blocking,
};

type Barrier = Arc<Notify>;

pub const SAMPLE_RATE: u32 = 16000;

#[derive(Default, Builder, Debug)]
#[builder(setter(into), build_fn(validate = "Self::validate"))]
pub struct Whisper {
    language: Language,
    model: Model,
    #[builder(default = "false")]
    progress_bar: bool,
    #[builder(default = "false")]
    force_download: bool,
    #[builder(default = "false")]
    force_cpu: bool,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Download(#[from] hf_hub::api::tokio::ApiError),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    AudioDecoder(#[from] rodio::decoder::DecoderError),
    #[error("Unable to find duration")]
    AudioDuration,
    #[error(transparent)]
    ComputeBuilder(#[from] TranscribeBuilderError),
    #[error("Failed to initialize the tokenizer. Reason {0}")]
    Tokenizer(String),
    #[error(transparent)]
    BurnConfig(#[from] burn::config::ConfigError),
    #[error(transparent)]
    BurnRecorder(#[from] burn::record::RecorderError),
}

#[derive(Clone, Debug, Display, EnumIs)]
pub enum Event {
    #[strum(to_string = "Downloading {file}")]
    DownloadStarted { file: String },
    #[strum(to_string = "{file} has been downloaded")]
    DownloadCompleted { file: String },
    #[strum(to_string = "{transcription}")]
    Segment {
        start_offset: Duration,
        end_offset: Duration,
        percentage: f32,
        transcription: String,
    },
}

impl WhisperBuilder {
    fn validate(&self) -> Result<(), WhisperBuilderError> {
        if self.language.as_ref().is_some_and(|l| !l.is_english())
            && self.model.as_ref().is_some_and(|m| !m.is_multilingual())
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
    pub fn transcribe(self, path: impl AsRef<Path>) -> UnboundedReceiver<Result<Event, Error>> {
        let (tx, rx) = unbounded_channel();
        let (tx_event, mut rx_event) = unbounded_channel();

        let wait_download = Barrier::default();
        let download_completed = wait_download.clone();

        let path = path.as_ref().into();

        // Download events forwarder
        let tx_forwarder = tx.clone();
        spawn(async move {
            while let Some(msg) = rx_event.recv().await {
                let _ = tx_forwarder.send(Ok(msg));
            }
            wait_download.notify_one();
        });

        spawn(async move {
            // Download model data from Hugging Face
            let model = self
                .model
                .download_model_listener(self.progress_bar, self.force_download, tx_event)
                .await;
            download_completed.notified().await;

            spawn_blocking(move || {
                // Load audio file
                let audio = Self::load_audio(path);

                match audio.map(|audio| (audio, model)) {
                    Ok((audio, Ok(model_files))) => {
                        if self.force_cpu {
                            match TranscribeBuilder::<NdArray>::default()
                                .language(self.language)
                                .audio(audio)
                                .tx(tx.clone())
                                .try_model(model_files)
                                .unwrap()
                                .build()
                            {
                                Ok(compute) => compute.transcribe(),
                                Err(err) => {
                                    let _ = tx.send(Err(err.into()));
                                }
                            }
                        } else {
                            match TranscribeBuilder::<Wgpu>::default()
                                .language(self.language)
                                .audio(audio)
                                .tx(tx.clone())
                                .try_model(model_files)
                                .unwrap()
                                .build()
                            {
                                Ok(compute) => compute.transcribe(),
                                Err(err) => {
                                    let _ = tx.send(Err(err.into()));
                                }
                            }
                        }
                    }
                    Ok((_, Err(err))) => {
                        let _ = tx.send(Err(err));
                    }
                    Err(err) => {
                        let _ = tx.send(Err(err));
                    }
                }
            });
        });

        rx
    }

    fn load_audio(path: PathBuf) -> Result<(Vec<f32>, Duration), Error> {
        let reader = BufReader::new(File::open(path)?);
        let decoder = Decoder::new(reader)?;
        let duration = decoder.total_duration().ok_or(Error::AudioDuration)?;
        let resample: UniformSourceIterator<Decoder<BufReader<File>>, f32> =
            UniformSourceIterator::new(decoder, 1, SAMPLE_RATE);
        let samples = resample
            .low_pass(3000)
            .high_pass(200)
            .convert_samples()
            .collect::<Vec<f32>>();

        Ok((samples, duration))
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

    #[ignore]
    #[tokio::test]
    async fn simple_transcribe_ok() {
        let mut rx = WhisperBuilder::default()
            .language(Language::Italian)
            .model(Model::Tiny)
            .progress_bar(true)
            .force_cpu(false)
            .build()
            .unwrap()
            .transcribe(test_file!("samples_jfk.wav"));

        while let Some(msg) = rx.recv().await {
            assert!(msg.is_ok());
            println!("{msg:?}");
        }
    }
}
