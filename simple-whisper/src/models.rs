use std::path::PathBuf;

use cfg_if::cfg_if;
use hf_hub::{api::tokio::ApiRepo, Repo};
use strum::{Display, EnumIter, EnumString};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::{Error, Event};

/// OpenAI supported models
#[derive(Default, Clone, Debug, EnumIter, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Model {
    /// The tiny model.
    #[strum(serialize = "tiny", to_string = "Tiny - tiny")]
    Tiny,
    /// The tiny model with only English support.
    #[strum(serialize = "tiny_en", to_string = "TinyEn - tiny_en")]
    TinyEn,
    /// The base model.
    #[default]
    #[strum(serialize = "base", to_string = "Base - base")]
    Base,
    /// The base model with only English support.
    #[strum(serialize = "base_en", to_string = "BaseEn - base_en")]
    BaseEn,
    /// The small model.
    #[strum(serialize = "small", to_string = "Small - small")]
    Small,
    /// The small model with only English support.
    #[strum(serialize = "small_en", to_string = "SmallEn - small_en")]
    SmallEn,
    /// The medium model.
    #[strum(serialize = "medium", to_string = "Medium - medium")]
    Medium,
    /// The medium model with only English support.
    #[strum(serialize = "medium_en", to_string = "MediumEn - medium_en")]
    MediumEn,
    /// The large model.
    #[strum(serialize = "large", to_string = "Large V1 - large")]
    Large,
    /// The large model v2.
    #[strum(serialize = "large_v2", to_string = "Large V2 - large_v2")]
    LargeV2,
    /// The large model v3.
    #[strum(serialize = "large_v3", to_string = "Large V3 - large_v3")]
    LargeV3,
}
struct HFCoordinates {
    repo: Repo,
    config: Option<String>,
    model: String,
    tokenizer: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LocalModel {
    pub config: Option<PathBuf>,
    pub model: PathBuf,
    pub tokenizer: Option<PathBuf>,
    pub model_type: Model,
}

impl Model {
    cfg_if! {
        if #[cfg(feature = "burn_vulkan")] {
            fn hf_coordinates(&self) -> HFCoordinates {
                let repo = Repo::with_revision(
                    "newfla/simple-whisper".to_owned(),
                    hf_hub::RepoType::Model,
                    "main".to_owned(),
                );
                match self {
                    Model::Tiny => HFCoordinates {
                        repo,
                        config: Some("tiny/tiny.cfg".to_owned()),
                        model: "tiny/tiny.mpk".to_owned(),
                        tokenizer: Some("tiny/tokenizer.json".to_owned()),
                    },
                    Model::TinyEn => HFCoordinates {
                        repo,
                        config: Some("tiny_en/tiny_en.cfg".to_owned()),
                        model: "tiny_en/tiny_en.mpk".to_owned(),
                        tokenizer: Some("tiny_en/tokenizer.json".to_owned()),
                    },
                    Model::Base => HFCoordinates {
                        repo,
                        config: Some("base/base.cfg".to_owned()),
                        model: "base/base.mpk".to_owned(),
                        tokenizer: Some("base/tokenizer.json".to_owned()),
                    },
                    Model::BaseEn => HFCoordinates {
                        repo,
                        config: Some("base_en/base_en.cfg".to_owned()),
                        model: "base_en/base_en.mpk".to_owned(),
                        tokenizer: Some("tiny/tokenizer.json".to_owned()),
                    },
                    Model::Small => HFCoordinates {
                        repo,
                        config: Some("small/small.cfg".to_owned()),
                        model: "small/small.mpk".to_owned(),
                        tokenizer: Some("small/tokenizer.json".to_owned()),
                    },
                    Model::SmallEn => HFCoordinates {
                        repo,
                        config: Some("small_en/small_en.cfg".to_owned()),
                        model: "small_en/small_en.mpk".to_owned(),
                        tokenizer: Some("small_en/tokenizer.json".to_owned()),
                    },
                    Model::Medium => HFCoordinates {
                        repo,
                        config: Some("medium/medium.cfg".to_owned()),
                        model: "medium/medium.mpk".to_owned(),
                        tokenizer: Some("medium/tokenizer.json".to_owned()),
                    },
                    Model::MediumEn => HFCoordinates {
                        repo,
                        config: Some("medium_en/medium_en.cfg".to_owned()),
                        model: "medium_en/medium_en.mpk".to_owned(),
                        tokenizer: Some("medium_en/tokenizer.json".to_owned()),
                    },
                    Model::Large => HFCoordinates {
                        repo,
                        config: Some("large-v1/large-v1.cfg".to_owned()),
                        model: "large-v1/large-v1.mpk".to_owned(),
                        tokenizer: Some("large-v1/tokenizer.json".to_owned()),
                    },
                    Model::LargeV2 => HFCoordinates {
                        repo,
                        config: Some("large-v2/large-v2.cfg".to_owned()),
                        model: "large-v2/large-v2.mpk".to_owned(),
                        tokenizer: Some("large-v2/tokenizer.json".to_owned()),
                    },
                    Model::LargeV3 => HFCoordinates {
                        repo,
                        config: Some("large-v3/large-v3.cfg".to_owned()),
                        model: "large-v3/large-v3.mpk".to_owned(),
                        tokenizer: Some("large-v3/tokenizer.json".to_owned()),
                    },
                }
            }
        } else if #[cfg(feature = "whisper_cpp_vulkan")] {
            fn hf_coordinates(&self) -> HFCoordinates {
                let repo = Repo::with_revision(
                    "ggerganov/whisper.cpp".to_owned(),
                    hf_hub::RepoType::Model,
                    "main".to_owned(),
                );
                match self {
                    Model::Tiny => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-tiny.bin".to_owned(),
                        tokenizer: None,
                    },
                    Model::TinyEn => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-tiny.en.bin".to_owned(),
                        tokenizer: None,
                    },
                    Model::Base => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-base.bin".to_owned(),
                        tokenizer: None,
                    },
                    Model::BaseEn => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-base.en.bin".to_owned(),
                        tokenizer: None,
                    },
                    Model::Small => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-small.bin".to_owned(),
                        tokenizer: None,
                    },
                    Model::SmallEn => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-small.en.bin".to_owned(),
                        tokenizer: None,
                    },
                    Model::Medium => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-medium.bin".to_owned(),
                        tokenizer: None,
                    },
                    Model::MediumEn => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-medium.en.bin".to_owned(),
                        tokenizer: None,
                    },
                    Model::Large => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-large-v1.bin".to_owned(),
                        tokenizer: None,
                    },
                    Model::LargeV2 => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-large-v2.bin".to_owned(),
                        tokenizer: None,
                    },
                    Model::LargeV3 => HFCoordinates {
                        repo,
                        config: None,
                        model: "ggml-large-v3.bin".to_owned(),
                        tokenizer: None,
                    },
                }
            }
        }

    }

    pub fn is_multilingual(&self) -> bool {
        !self.to_string().contains("en")
    }

    pub async fn download_model_listener(
        &self,
        progress: bool,
        force_download: bool,
        tx: UnboundedSender<Event>,
    ) -> Result<LocalModel, Error> {
        let coordinates = self.hf_coordinates();
        let repo = hf_hub::api::tokio::ApiBuilder::default()
            .with_progress(progress)
            .build()
            .map(|api| api.repo(coordinates.repo))
            .map_err(Into::<Error>::into)?;
        let tokenizer = if let Some(val) = coordinates.tokenizer {
            Some(download_file(&val, force_download, &tx, &repo).await?)
        } else {
            None
        };
        let config = if let Some(val) = coordinates.config {
            Some(download_file(&val, force_download, &tx, &repo).await?)
        } else {
            None
        };
        let model = download_file(&coordinates.model, force_download, &tx, &repo).await?;
        Ok(LocalModel {
            config,
            model,
            tokenizer,
            model_type: self.clone(),
        })
    }

    pub async fn download_model(
        &self,
        progress: bool,
        force_download: bool,
    ) -> Result<LocalModel, Error> {
        let (tx, _rx) = unbounded_channel();
        self.download_model_listener(progress, force_download, tx)
            .await
    }
}

async fn download_file(
    file: &str,
    force_download: bool,
    tx: &UnboundedSender<Event>,
    repo: &ApiRepo,
) -> Result<PathBuf, Error> {
    let _ = tx.send(Event::DownloadStarted {
        file: file.to_owned(),
    });
    match force_download {
        false => repo.get(file).await,
        true => repo.download(file).await,
    }
    .map_err(Into::into)
    .map(|val| {
        let _ = tx.send(Event::DownloadCompleted {
            file: file.to_owned(),
        });
        val
    })
}
