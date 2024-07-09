use std::path::PathBuf;

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
    config: String,
    model: String,
    tokenizer: String,
}

#[derive(Debug, Clone)]
pub struct LocalModel {
    pub config: PathBuf,
    pub model: PathBuf,
    pub tokenizer: PathBuf,
}

impl Model {
    fn hf_coordinates(&self) -> HFCoordinates {
        let repo = Repo::with_revision(
            "newfla/simple-whisper".to_owned(),
            hf_hub::RepoType::Model,
            "main".to_owned(),
        );
        match self {
            Model::Tiny => HFCoordinates {
                repo,
                config: "tiny/tiny.cfg".to_owned(),
                model: "tiny/tiny.mpk".to_owned(),
                tokenizer: "tiny/tokenizer.json".to_owned(),
            },
            Model::TinyEn => HFCoordinates {
                repo,
                config: "tiny_en/tiny_en.cfg".to_owned(),
                model: "tiny_en/tiny_en.mpk".to_owned(),
                tokenizer: "tiny_en/tokenizer.json".to_owned(),
            },
            Model::Base => HFCoordinates {
                repo,
                config: "base/base.cfg".to_owned(),
                model: "base/base.mpk".to_owned(),
                tokenizer: "base/tokenizer.json".to_owned(),
            },
            Model::BaseEn => HFCoordinates {
                repo,
                config: "base_en/base_en.cfg".to_owned(),
                model: "base_en/base_en.mpk".to_owned(),
                tokenizer: "tiny/tokenizer.json".to_owned(),
            },
            Model::Small => HFCoordinates {
                repo,
                config: "small/small.cfg".to_owned(),
                model: "small/small.mpk".to_owned(),
                tokenizer: "small/tokenizer.json".to_owned(),
            },
            Model::SmallEn => HFCoordinates {
                repo,
                config: "small_en/small_en.cfg".to_owned(),
                model: "small_en/small_en.mpk".to_owned(),
                tokenizer: "small_en/tokenizer.json".to_owned(),
            },
            Model::Medium => HFCoordinates {
                repo,
                config: "medium/medium.cfg".to_owned(),
                model: "medium/medium.mpk".to_owned(),
                tokenizer: "medium/tokenizer.json".to_owned(),
            },
            Model::MediumEn => HFCoordinates {
                repo,
                config: "medium_en/medium_en.cfg".to_owned(),
                model: "medium_en/medium_en.mpk".to_owned(),
                tokenizer: "medium_en/tokenizer.json".to_owned(),
            },
            Model::Large => HFCoordinates {
                repo,
                config: "large-v1/large-v1.cfg".to_owned(),
                model: "large-v1/large-v1.mpk".to_owned(),
                tokenizer: "large-v1/tokenizer.json".to_owned(),
            },
            Model::LargeV2 => HFCoordinates {
                repo,
                config: "large-v2/large-v2.cfg".to_owned(),
                model: "large-v2/large-v2.mpk".to_owned(),
                tokenizer: "large-v2/tokenizer.json".to_owned(),
            },
            Model::LargeV3 => HFCoordinates {
                repo,
                config: "large-v3/large-v3.cfg".to_owned(),
                model: "large-v3/large-v3.mpk".to_owned(),
                tokenizer: "large-v3/tokenizer.json".to_owned(),
            },
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
        let tokenizer = download_file(&coordinates.tokenizer, force_download, &tx, &repo).await?;
        let config = download_file(&coordinates.config, force_download, &tx, &repo).await?;
        let model = download_file(&coordinates.model, force_download, &tx, &repo).await?;
        Ok(LocalModel {
            config,
            model,
            tokenizer,
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
