use std::time::Duration;

use derive_builder::Builder;
use tokio::sync::mpsc::UnboundedSender;
use whisper_rs::{
    DtwMode::ModelPreset, DtwModelPreset, FullParams, SamplingStrategy, SegmentCallbackData,
    WhisperContext, WhisperContextParameters, WhisperError, WhisperState,
};

use crate::{models::LocalModel, Error, Event, Language};

#[derive(Builder)]
#[builder(
    setter(into),
    pattern = "owned",
    build_fn(skip, error = "TranscribeBuilderError")
)]
pub struct Transcribe {
    language: Language,
    #[builder(setter(name = "force_cpu"))]
    _force_cpu: bool,
    audio: (Vec<f32>, Duration),
    tx: UnboundedSender<Result<Event, Error>>,
    #[builder(setter(name = "model"))]
    _model: LocalModel,
    #[builder(setter(skip))]
    state: WhisperState,
}

impl TranscribeBuilder {
    pub fn build(self) -> Result<Transcribe, TranscribeBuilderError> {
        if self.language.is_none() {
            return Err(TranscribeBuilderError::UninitializedFieldError("language"));
        }

        if self._force_cpu.is_none() {
            return Err(TranscribeBuilderError::UninitializedFieldError("force_cpu"));
        }

        if self.audio.is_none() {
            return Err(TranscribeBuilderError::UninitializedFieldError("audio"));
        }

        if self.tx.is_none() {
            return Err(TranscribeBuilderError::UninitializedFieldError("tx"));
        }

        if self._model.is_none() {
            return Err(TranscribeBuilderError::UninitializedFieldError("model"));
        }

        let state = state_builder(self._model.as_ref().unwrap(), self._force_cpu.unwrap())?;

        Ok(Transcribe {
            language: self.language.unwrap(),
            _force_cpu: self._force_cpu.unwrap(),
            audio: self.audio.unwrap(),
            tx: self.tx.unwrap(),
            _model: self._model.unwrap(),
            state,
        })
    }
}

#[derive(Error, Debug)]
pub enum TranscribeBuilderError {
    #[error("Field not initialized: {0}")]
    UninitializedFieldError(&'static str),
    #[error(transparent)]
    WhisperCppError(#[from] WhisperError),
}

fn state_builder(model: &LocalModel, force_cpu: bool) -> Result<WhisperState, WhisperError> {
    let mut context_param = WhisperContextParameters::default();
    context_param.dtw_parameters.mode = ModelPreset {
        model_preset: match model.model_type {
            crate::Model::Tiny => DtwModelPreset::Tiny,
            crate::Model::TinyEn => DtwModelPreset::TinyEn,
            crate::Model::Base => DtwModelPreset::Base,
            crate::Model::BaseEn => DtwModelPreset::BaseEn,
            crate::Model::Small => DtwModelPreset::Small,
            crate::Model::SmallEn => DtwModelPreset::SmallEn,
            crate::Model::Medium => DtwModelPreset::Medium,
            crate::Model::MediumEn => DtwModelPreset::MediumEn,
            crate::Model::Large => DtwModelPreset::LargeV1,
            crate::Model::LargeV2 => DtwModelPreset::LargeV2,
            crate::Model::LargeV3 => DtwModelPreset::LargeV3,
        },
    };

    context_param.use_gpu(!force_cpu);

    let ctx = WhisperContext::new_with_params(model.model.to_str().unwrap(), context_param)?;

    ctx.create_state()
}

impl Transcribe {
    pub fn transcribe(mut self) {
        let tx_callback = self.tx.downgrade();

        let (audio, duration) = &self.audio;
        let duration = duration.clone();
        let lang = self.language.to_string();

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
        params.set_language(Some(&lang));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_timestamps(false);

        params.set_segment_callback_safe(move |seg: SegmentCallbackData| {
            let start_offset = Duration::from_millis(seg.start_timestamp as u64 * 10);
            let end_offset = Duration::from_millis(seg.end_timestamp as u64 * 10);
            let mut percentage = end_offset.as_millis() as f32 / duration.as_millis() as f32;
            if percentage > 1. {
                percentage = 1.;
            }
            let seg = Event::Segment {
                start_offset,
                end_offset,
                percentage,
                transcription: seg.text,
            };
            let _ = tx_callback.upgrade().unwrap().send(Ok(seg));
        });

        if let Err(err) = self.state.full(params, &audio) {
            let _ = self.tx.send(Err(Error::WhisperCppError(err)));
        }
    }
}
