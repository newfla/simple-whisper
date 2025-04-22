use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Instant,
};

use hf_hub::{Cache, Repo, api::tokio::Progress};
use tokio::sync::mpsc::UnboundedSender;

use crate::{Error, Event};

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

pub enum ProgressType {
    Callback(UnboundedSender<Event>),
    ProgressBar,
}

pub async fn download_file(
    file: &str,
    force_download: bool,
    progress: ProgressType,
    repo: Repo,
) -> Result<PathBuf, Error> {
    let cache = Cache::from_env().repo(repo.clone());
    let mut in_cache = cache.get(file);
    if force_download {
        in_cache = None
    }
    if let Some(val) = in_cache {
        Ok(val)
    } else {
        match progress {
            ProgressType::ProgressBar => {
                hf_hub::api::tokio::ApiBuilder::default()
                    .with_progress(true)
                    .build()
                    .map(|api| api.repo(repo))
                    .map_err(Into::<Error>::into)?
                    .download(file)
                    .await
            }
            ProgressType::Callback(tx) => {
                let progress = DownloadCallback {
                    download_state: Default::default(),
                    tx,
                };
                hf_hub::api::tokio::ApiBuilder::default()
                    .with_progress(false)
                    .build()
                    .map(|api| api.repo(repo))
                    .map_err(Into::<Error>::into)?
                    .download_with_progress(file, progress)
                    .await
            }
        }
        .map_err(Into::into)
    }
}
