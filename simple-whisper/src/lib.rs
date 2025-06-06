use std::{
    fs::File,
    io::{self, BufReader},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use derive_builder::Builder;

mod download;
mod language;
mod model;
mod transcribe;

use download::ProgressType;
pub use language::Language;
pub use model::Model;
use rodio::{Decoder, Source, source::UniformSourceIterator};
use strum::{Display, EnumIs};
use thiserror::Error;
use tokio::{
    spawn,
    sync::{Notify, mpsc::unbounded_channel},
    task::spawn_blocking,
};
pub use transcribe::TranscribeBuilderError;

use tokio_stream::{Stream, wrappers::UnboundedReceiverStream};
use transcribe::TranscribeBuilder;
use whisper_rs::WhisperError;

type Barrier = Arc<Notify>;

pub const SAMPLE_RATE: u32 = 16000;

/// The Whisper audio transcription model.
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
    force_single_segment: bool,
}

/// Error conditions
#[derive(Error, Debug)]
pub enum Error {
    /// Error that can occur during model files download from huggingface
    #[error(transparent)]
    Download(#[from] hf_hub::api::tokio::ApiError),
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Error that can occur during audio file decoding phase
    #[error(transparent)]
    AudioDecoder(#[from] rodio::decoder::DecoderError),
    /// The library was unable to determine the audio file duration
    #[error("Unable to find duration")]
    AudioDuration,
    #[error(transparent)]
    /// Missing parameters to instantiate the whisper cpp backend
    ComputeBuilder(#[from] TranscribeBuilderError),
    #[error(transparent)]
    Whisper(#[from] WhisperError),
}

/// Events generated by the [Whisper::transcribe] method
#[derive(Clone, Debug, Display, EnumIs)]
pub enum Event {
    #[strum(to_string = "Downloading {file}")]
    DownloadStarted { file: String },
    #[strum(to_string = "{file} has been downloaded")]
    DownloadCompleted { file: String },
    #[strum(
        to_string = "Downloading {file} --> {percentage} {elapsed_time:#?} | {remaining_time:#?}"
    )]
    DownloadProgress {
        /// The resource to download
        file: String,

        /// The progress expressed as %
        percentage: f32,

        /// Time elapsed since the download as being started
        elapsed_time: Duration,

        /// Estimated time to complete the download
        remaining_time: Duration,
    },
    /// Audio chunk transcript
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
    /// Transcribe an audio file into text.
    pub fn transcribe(self, path: impl AsRef<Path>) -> impl Stream<Item = Result<Event, Error>> {
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
            let progress = if self.progress_bar {
                drop(tx_event);
                ProgressType::ProgressBar
            } else {
                ProgressType::Callback(tx_event)
            };
            let model = self
                .model
                .internal_download_model(self.force_download, progress)
                .await;
            download_completed.notified().await;

            spawn_blocking(move || {
                // Load audio file
                let audio = Self::load_audio(path);

                match audio.map(|audio| (audio, model)) {
                    Ok((audio, Ok(model_files))) => {
                        match TranscribeBuilder::default()
                            .language(self.language)
                            .audio(audio)
                            .single_segment(self.force_single_segment)
                            .tx(tx.clone())
                            .model(model_files)
                            .build()
                        {
                            Ok(compute) => compute.transcribe(),
                            Err(err) => {
                                let _ = tx.send(Err(err.into()));
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

        UnboundedReceiverStream::new(rx)
    }

    fn load_audio(path: PathBuf) -> Result<(Vec<f32>, Duration), Error> {
        let reader = BufReader::new(File::open(&path)?);
        let decoder = Decoder::new(reader)?;
        let resample: UniformSourceIterator<Decoder<BufReader<File>>, f32> =
            UniformSourceIterator::new(decoder, 1, SAMPLE_RATE);
        let samples = resample
            .low_pass(3000)
            .high_pass(200)
            .convert_samples()
            .collect::<Vec<f32>>();

        let duration = Self::get_audio_duration(samples.len());

        Ok((samples, duration))
    }

    fn get_audio_duration(samples: usize) -> Duration {
        let secs = samples as f64 / SAMPLE_RATE as f64;
        Duration::from_secs_f64(secs)
    }
}

#[cfg(test)]
mod tests {
    use tokio_stream::StreamExt;

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
            .language(Language::English)
            .model(Model::Tiny)
            .progress_bar(true)
            .build()
            .unwrap()
            .transcribe(test_file!("samples_jfk.wav"));

        while let Some(msg) = rx.next().await {
            assert!(msg.is_ok());
            println!("{msg:?}");
        }
    }
}
