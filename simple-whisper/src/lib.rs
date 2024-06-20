use std::path::Path;

use derive_builder::Builder;

mod lang;
mod model;

pub use lang::Language;
pub use model::Model;
use thiserror::Error;

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
    pub async fn transcribe(self, file: impl AsRef<Path>) -> Result<(), Error> {
        let _local_model = self
            .model
            .download_model(self.progress_bar, self.force_download)
            .await?;

        Ok(())
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
        assert!(WhisperBuilder::default()
            .language(Language::Italian)
            .model(Model::Tiny)
            .progress_bar(true)
            .build()
            .unwrap()
            .transcribe(test_file!("samples_jfk.mp3"))
            .await
            .is_ok());
    }
}
