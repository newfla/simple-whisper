use std::{iter::once, ops::Div};

use burn::{
    backend::{ndarray::NdArrayDevice, wgpu::WgpuDevice, NdArray, Wgpu},
    config::Config,
    module::Module,
    record::{FullPrecisionSettings, NamedMpkFileRecorder, Recorder},
    tensor::{activation::log_softmax, backend::Backend, Data, ElementConversion, Tensor},
};
use derive_builder::Builder;
use tokenizers::Tokenizer;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    burn_impl::{
        audio::{max_waveform_samples, prep_audio},
        beam::{self, beam_search},
        token::SpecialToken,
        whisper::{WhisperModel, WhisperModelConfig},
    },
    models::LocalModel,
    Error, Event, Language, SAMPLE_RATE,
};

const PADDING: usize = 200;
const CHUNK_OVERLAP: u32 = SAMPLE_RATE * 0; //0 --> disable overlapping

#[derive(Builder, Debug)]
#[builder(setter(into))]
pub struct Transcribe<B: Backend> {
    language: Language,
    audio: Vec<f32>,
    tx: UnboundedSender<Result<Event, Error>>,
    #[builder(try_setter, setter(into, name = "model"))]
    model_impl: ModelImpl<B>,
}

#[derive(Clone, Debug)]
pub(crate) struct ModelImpl<B: Backend> {
    tokenizer: Tokenizer,
    model: WhisperModel<B>,
    cfg: WhisperModelConfig,
}

impl<B: Backend> ModelImpl<B> {
    fn transcribe(self, waveform: Vec<f32>, lang: Language) -> impl Iterator<Item = Result<Event,Error>> {
        let init_tokens = self.init_token(lang);
        let (tot, mels) = self.waveform_to_mel_tensor(waveform);
        mels.enumerate().map(move |(idx, mel)| {
            self.mels_to_text(init_tokens.clone(), mel).map(|transcription| {
                Event::Segment {
                    start_offset: 0.,
                    end_offset: 0.,
                    percentage: (idx + 1) as f32 / tot as f32,
                    transcription,
                }
            })
        })
    }

    fn waveform_to_mel_tensor(
        &self,
        waveform: Vec<f32>,
    ) -> (usize, impl Iterator<Item = Tensor<B, 3>>) {
        let device = self.model.devices()[0].clone();
        let window_length_samples =
            max_waveform_samples(self.cfg.audio_encoder_ctx_size() - PADDING);
        let n_mels = self.cfg.audio_encoder_mel_size();

        let n_samples_per_tensor = window_length_samples;
        let shift = n_samples_per_tensor
            .saturating_sub(CHUNK_OVERLAP as usize)
            .max(1);
        let iter_len = waveform.len().saturating_sub(1).div(shift) + 1;

        (
            iter_len,
            (0..iter_len).map(move |i| {
                let start = i * shift;
                let end = (start + n_samples_per_tensor).min(waveform.len());

                let slice = &waveform[start..end];

                let waveform =
                    Tensor::from_floats(Data::new(slice.to_vec(), [slice.len()].into()), &device);

                prep_audio(waveform.unsqueeze(), SAMPLE_RATE as f64, n_mels)
            }),
        )
    }

    fn mels_to_text(&self, token_utility: TokenUtility<B>, mels: Tensor<B, 3>) -> Result<String, Error> {
        let device = mels.device();

        let n_ctx_max_encoder = self.cfg.audio_encoder_ctx_size();

        let [_, n_mel, n_ctx] = mels.dims();

        // the zero padding helps whisper determine end of text
        let mels = Tensor::cat(
            vec![
                mels.slice([0..1, 0..n_mel, 0..(n_ctx).min(n_ctx_max_encoder - PADDING)]),
                Tensor::zeros([1, n_mel, PADDING], &device),
            ],
            2,
        );
        let encoder_output = self.model.forward_encoder(mels);

        type BeamNode = beam::BeamNode<usize>;
        let initial_tokens = BeamNode {
            seq: token_utility.inital_tokens,
            log_prob: 0.0,
        };

        let beamsearch_next = |beams: &[BeamNode]| {
            // convert tokens into tensor
            let max_seq_len = beams.iter().map(|beam| beam.seq.len()).max().unwrap_or(0);
            let flattened_tokens: Vec<_> = beams
                .iter()
                .flat_map(|beam| {
                    let additional_tokens = max_seq_len - beam.seq.len();
                    beam.seq
                        .iter()
                        .copied()
                        .chain(once(0).cycle().take(additional_tokens))
                })
                .collect();

            let token_tensor = Tensor::from_ints(
                Data::from_usize(Data::new(
                    flattened_tokens,
                    [beams.len(), max_seq_len].into(),
                )),
                &device,
            );

            let logits = self
                .model
                .forward_decoder(token_tensor, encoder_output.clone().repeat(0, beams.len()));
            let logits = if max_seq_len > 5 {
                logits
            } else {
                logits + token_utility.special_tokens_maskout.unsqueeze()
            };
            let log_probs = log_softmax(logits, 2);

            let beam_log_probs = beams.iter().enumerate().map(|(i, beam)| {
                let batch = i;
                let token_index = beam.seq.len() - 1;

                log_probs
                    .clone()
                    .slice([batch..batch + 1, token_index..token_index + 1])
                    .flatten::<1>(0, 2)
                    .into_data()
                    .value
            });

            beam_log_probs
                .zip(beams)
                .map(|(log_probs, beam)| {
                    log_probs
                        .into_iter()
                        .map(|log_prob| log_prob.elem::<f64>())
                        .enumerate()
                        .map(|(token_id, log_prob)| (token_id, beam.log_prob + log_prob))
                        .collect()
                })
                .collect()
        };

        let beamsearch_is_finished = |toks: &[usize]| {
            if let Some(btok) = toks.last() {
                *btok == token_utility.end_token
            } else {
                false
            }
        };

        let beam_size = 5;
        let max_depth = 30;
        let tokens: Vec<_> = beam_search(
            vec![initial_tokens],
            beamsearch_next,
            beamsearch_is_finished,
            beam_size,
            max_depth,
        );

        self.decode(&tokens[..], true)
    }

    fn init_token(&self, lang: Language) -> TokenUtility<B> {
        let start_token = self.special_token(SpecialToken::StartofTranscript).unwrap();
        let transcription_token = self.special_token(SpecialToken::Transcribe).unwrap();
        let lang_token = self.special_token(SpecialToken::Language(lang)).unwrap();
        let end_token = self.special_token(SpecialToken::EndofText).unwrap();
        let notimestamp = self.special_token(SpecialToken::NoTimeStamps).unwrap();

        let vec = vec![start_token, lang_token, transcription_token, notimestamp];

        let vocab_size = self.tokenizer.get_vocab_size(true);

        let special_tokens_maskout: Vec<f32> = (0..vocab_size)
            .map(|token| {
                if self.is_special(token) {
                    -f32::INFINITY
                } else {
                    0.
                }
            })
            .collect();

        let device = self.model.devices()[0].clone();

        let special_tokens_maskout = Tensor::from_data(
            Data::new(special_tokens_maskout, [vocab_size].into()).convert(),
            &device,
        );

        TokenUtility {
            inital_tokens: vec,
            end_token,
            special_tokens_maskout,
        }
    }

    fn special_token(&self, token: SpecialToken) -> Option<usize> {
        self.tokenizer
            .token_to_id(&token.to_string())
            .map(|t| t as usize)
    }

    fn decode(&self, tokens: &[usize], skip_special: bool) -> Result<String, Error> {
        self.tokenizer
            .decode(
                &tokens.iter().map(|t| *t as u32).collect::<Vec<u32>>(),
                skip_special,
            )
            .map_err(|err| Error::Tokenizer(err.to_string()))
    }

    fn is_special(&self, token: usize) -> bool {
        self.tokenizer
            .decode(vec![token as u32].as_slice(), true)
            .ok()
            .map(|s| s.is_empty())
            .unwrap_or(false)
    }

    fn load_model(value: LocalModel, device: B::Device) -> Result<Self, Error> {
        let tokenizer = tokenizer(&value)?;

        let cfg = WhisperModelConfig::load(value.config)?;
        let model = NamedMpkFileRecorder::<FullPrecisionSettings>::new()
            .load(value.model, &device)
            .map(|record| cfg.init(&device).load_record(record))?;

        Ok(ModelImpl {
            tokenizer,
            model,
            cfg,
        })
    }
}

impl TryFrom<LocalModel> for ModelImpl<NdArray> {
    type Error = Error;

    fn try_from(value: LocalModel) -> Result<Self, Self::Error> {
        let device = NdArrayDevice::Cpu;
        ModelImpl::load_model(value, device)
    }
}

impl TryFrom<LocalModel> for ModelImpl<Wgpu> {
    type Error = Error;

    fn try_from(value: LocalModel) -> Result<Self, Self::Error> {
        let device = WgpuDevice::default();
        ModelImpl::load_model(value, device)
    }
}

impl<B: Backend> Transcribe<B> {
    pub fn transcribe(self) {
        for msg in self.model_impl.transcribe(self.audio, self.language) {
            if self.tx.send(msg).is_err() {
                break;
            }
        }
    }
}

#[derive(Clone)]
struct TokenUtility<B: Backend> {
    inital_tokens: Vec<usize>,
    end_token: usize,
    special_tokens_maskout: Tensor<B, 1>,
}

fn tokenizer(value: &LocalModel) -> Result<Tokenizer, Error> {
    Tokenizer::from_file(&value.tokenizer).map_err(|err| Error::Tokenizer(err.to_string()))
}
