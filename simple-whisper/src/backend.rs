use std::time::Duration;

use tokio::sync::mpsc::UnboundedSender;

use crate::{model::LocalModel, Error, Event, Language};

pub (crate) struct Compute {
    language: Language,
    model_files: LocalModel,
    audio: Vec<f32>,
    tx: UnboundedSender<Result<Event, Error>>
}

impl Compute {
    pub (crate) fn new(language: Language,
        model_files: LocalModel,
        audio: Vec<f32>,
        tx: UnboundedSender<Result<Event, Error>>) ->  Self {
            Self {
                language,
                model_files,
                audio,
                tx,
            }

    }
    pub (crate) async fn compute(self) {
        //Stub send
        let _ = self.tx.send(Ok(Event::Segment {
            start_offset: 0.,
            end_offset: 0.,
            percentage: 0.,
            transcription: "Stub0".to_owned(),
        }));

        tokio::time::sleep(Duration::from_secs(2)).await;
        let _ = self.tx.send(Ok(Event::Segment {
            start_offset: 0.,
            end_offset: 0.,
            percentage: 0.5,
            transcription: "Stub1".to_owned(),
        }));
        tokio::time::sleep(Duration::from_secs(2)).await;
        let _ = self.tx.send(Ok(Event::Segment {
            start_offset: 0.,
            end_offset: 0.,
            percentage: 1.,
            transcription: "Stub2".to_owned(),
        }));
    }
}
