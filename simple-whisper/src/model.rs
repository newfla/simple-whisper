use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Instant,
};

use hf_hub::{
    api::tokio::{ApiRepo, Progress},
    Cache, CacheRepo, Repo,
};
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
            .map(|api| api.repo(coordinates.repo.clone()))
            .map_err(Into::<Error>::into)?;
        let cache = Cache::from_env().repo(coordinates.repo);
        let tokenizer = if let Some(val) = coordinates.tokenizer {
            Some(download_file(&val, force_download, &tx, &repo, &cache).await?)
        } else {
            None
        };
        let config = if let Some(val) = coordinates.config {
            Some(download_file(&val, force_download, &tx, &repo, &cache).await?)
        } else {
            None
        };
        let model = download_file(&coordinates.model, force_download, &tx, &repo, &cache).await?;
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
    cache: &CacheRepo,
) -> Result<PathBuf, Error> {
    let mut in_cache = cache.get(file);
    if force_download {
        in_cache = None
    }
    if let Some(val) = in_cache {
        Ok(val)
    } else {
        let progress = DownloadCallback {
            download_state: Default::default(),
            tx: tx.clone(),
        };
        repo.download_with_progress(file, progress)
            .await
            .map_err(Into::into)
    }
}

/// Store the state of a download
#[derive(Debug, Clone)]
struct DownloadState {
    start_time: Instant,
    len: usize,
    offset: usize,
    url: String,
}

impl DownloadState {
    fn new(len: usize, url: &str) -> DownloadState {
        DownloadState {
            start_time: Instant::now(),
            len,
            offset: 0,
            url: url.to_string(),
        }
    }

    fn update(&mut self, delta: usize) -> Option<Event> {
        if delta == 0 {
            return None;
        }

        self.offset += delta;

        let elapsed_time = Instant::now() - self.start_time;

        let progress = self.offset as f32 / self.len as f32;
        let progress_100 = progress * 100.;

        let remaining_percentage = 100. - progress_100;
        let duration_unit = elapsed_time
            / if progress_100 as u32 == 0 {
                1
            } else {
                progress_100 as u32
            };
        let remaining_time = duration_unit * remaining_percentage as u32;

        let event = Event::DownloadProgress {
            file: self.url.clone(),
            percentage: progress_100,
            elapsed_time,
            remaining_time,
        };
        Some(event)
    }
}

#[derive(Clone)]
struct DownloadCallback {
    download_state: Arc<Mutex<Option<DownloadState>>>,
    tx: UnboundedSender<Event>,
}

impl Progress for DownloadCallback {
    async fn init(&mut self, len: usize, file: &str) {
        self.download_state = Arc::new(Mutex::new(Some(DownloadState::new(len, file))));

        let _ = self.tx.send(Event::DownloadStarted {
            file: file.to_owned(),
        });
    }

    async fn update(&mut self, delta: usize) {
        let update = self
            .download_state
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .update(delta);
        if let Some(event) = update {
            let _ = self.tx.send(event);
        }
    }

    async fn finish(&mut self) {
        let file = self
            .download_state
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .url
            .clone();
        let _ = self.tx.send(Event::DownloadCompleted { file });
    }
}
