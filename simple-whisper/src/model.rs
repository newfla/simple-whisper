use std::path::PathBuf;

use hf_hub::{api::tokio::ApiRepo, Repo};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::{Error, Event, Model};

pub(crate) struct HFCoordinates {
    pub(crate) repo: Repo,
    pub(crate) config: Option<String>,
    pub(crate) model: String,
    pub(crate) tokenizer: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LocalModel {
    pub config: Option<PathBuf>,
    pub model: PathBuf,
    pub tokenizer: Option<PathBuf>,
    pub model_type: Model,
}
impl Model {
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
    .inspect(|_| {
        let _ = tx.send(Event::DownloadCompleted {
            file: file.to_owned(),
        });
    })
}
