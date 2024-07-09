use burn::{
    config::Config,
    module::{Module, Param},
    nn::{
        conv::{Conv1d, Conv1dConfig},
        Gelu, LayerNorm, LayerNormConfig, Linear, LinearConfig, PaddingConfig1d,
    },
    tensor::{activation::softmax, backend::Backend, module::embedding, Distribution, Int, Tensor},
};

#[derive(Config, Debug)]
pub struct WhisperModelConfig {
    audio_encoder_config: AudioEncoderConfig,
    text_decoder_config: TextDecoderConfig,
}

impl WhisperModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> WhisperModel<B> {
        WhisperModel {
            encoder: self.audio_encoder_config.init(device),
            decoder: self.text_decoder_config.init(device),
        }
    }

    pub fn audio_encoder_ctx_size(&self) -> usize {
        self.audio_encoder_config.n_audio_ctx
    }

    pub fn audio_encoder_mel_size(&self) -> usize {
        self.audio_encoder_config.n_mels
    }
}

#[derive(Module, Debug)]
pub struct WhisperModel<B: Backend> {
    encoder: AudioEncoder<B>,
    decoder: TextDecoder<B>,
}

impl<B: Backend> WhisperModel<B> {
    pub fn forward_encoder(&self, mel: Tensor<B, 3>) -> Tensor<B, 3> {
        self.encoder.forward(mel)
    }

    pub fn forward_decoder(
        &self,
        tokens: Tensor<B, 2, Int>,
        encoder_output: Tensor<B, 3>,
    ) -> Tensor<B, 3> {
        self.decoder.forward(tokens, encoder_output)
    }
}

#[derive(Config, Debug)]
struct AudioEncoderConfig {
    n_mels: usize,
    n_audio_ctx: usize,
    n_audio_state: usize,
    n_audio_head: usize,
    n_audio_layer: usize,
}

impl AudioEncoderConfig {
    fn init<B: Backend>(&self, device: &B::Device) -> AudioEncoder<B> {
        let conv1 = Conv1dConfig::new(self.n_mels, self.n_audio_state, 3)
            .with_padding(PaddingConfig1d::Explicit(1))
            .init(device);
        let gelu1 = Gelu::new();
        let conv2 = Conv1dConfig::new(self.n_audio_state, self.n_audio_state, 3)
            .with_padding(PaddingConfig1d::Explicit(1))
            .with_stride(2)
            .init(device);
        let gelu2 = Gelu::new();
        let blocks: Vec<_> = (0..self.n_audio_layer)
            .map(|_| {
                ResidualEncoderAttentionBlockConfig::new(self.n_audio_state, self.n_audio_head)
                    .init(device)
            })
            .collect();
        let ln_post = LayerNormConfig::new(self.n_audio_state).init(device);
        let positional_embedding = Param::from_tensor(Tensor::random(
            [self.n_audio_ctx, self.n_audio_state],
            Distribution::Normal(0.0, 1.0),
            device,
        ));
        let n_mels = self.n_mels;
        let n_audio_ctx = self.n_audio_ctx;

        AudioEncoder {
            conv1,
            gelu1,
            conv2,
            gelu2,
            blocks,
            ln_post,
            positional_embedding,
            n_mels,
            n_audio_ctx,
        }
    }
}

#[derive(Module, Debug)]
pub struct AudioEncoder<B: Backend> {
    conv1: Conv1d<B>,
    gelu1: Gelu,
    conv2: Conv1d<B>,
    gelu2: Gelu,
    blocks: Vec<ResidualEncoderAttentionBlock<B>>,
    ln_post: LayerNorm<B>,
    positional_embedding: Param<Tensor<B, 2>>,
    n_mels: usize,
    n_audio_ctx: usize,
}

impl<B: Backend> AudioEncoder<B> {
    fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 3> {
        let [_, n_mels, n_ctx] = x.dims();

        assert_eq!(n_mels, self.n_mels, "Audio mel spectrum size must be {}.", self.n_mels);
        assert!(
            n_ctx <= self.n_audio_ctx,
            "Audio length {} cannot exceed {}.",
            n_ctx,
            self.n_audio_ctx
        );

        let x = self.gelu1.forward(self.conv1.forward(x));
        let x = self.gelu2.forward(self.conv2.forward(x));

        let x = x.swap_dims(1, 2);
        let k = x.dims()[1];
        let x = x + self
            .positional_embedding
            .val()
            .slice([0..k])
            .unsqueeze::<3>();

        let mut x = x;
        for block in &self.blocks {
            x = block.forward(x);
        }

        self.ln_post.forward(x)
    }
}

#[derive(Config, Debug)]
struct TextDecoderConfig {
    n_vocab: usize,
    n_text_ctx: usize,
    n_text_state: usize,
    n_text_head: usize,
    n_text_layer: usize,
}

impl TextDecoderConfig {
    fn init<B: Backend>(&self, device: &B::Device) -> TextDecoder<B> {
        let token_embedding = Param::from_tensor(Tensor::random(
            [self.n_vocab, self.n_text_state],
            Distribution::Normal(0.0, 1.0),
            device,
        ));
        let positional_embedding = Param::from_tensor(Tensor::random(
            [self.n_text_ctx, self.n_text_state],
            Distribution::Normal(0.0, 1.0),
            device,
        ));
        let blocks: Vec<_> = (0..self.n_text_layer)
            .map(|_| {
                ResidualDecoderAttentionBlockConfig::new(self.n_text_state, self.n_text_head)
                    .init(device)
            })
            .collect();
        let ln = LayerNormConfig::new(self.n_text_state).init(device);

        let mask = Param::from_tensor(attn_decoder_mask(self.n_text_ctx, device));

        let n_text_ctx = self.n_text_ctx;

        TextDecoder {
            token_embedding,
            positional_embedding,
            blocks,
            ln,
            mask,
            n_text_ctx,
        }
    }
}

#[derive(Module, Debug)]
struct TextDecoder<B: Backend> {
    token_embedding: Param<Tensor<B, 2>>,
    positional_embedding: Param<Tensor<B, 2>>,
    blocks: Vec<ResidualDecoderAttentionBlock<B>>,
    ln: LayerNorm<B>,
    mask: Param<Tensor<B, 2>>,
    n_text_ctx: usize,
}

impl<B: Backend> TextDecoder<B> {
    fn forward(&self, x: Tensor<B, 2, Int>, xa: Tensor<B, 3>) -> Tensor<B, 3> {
        let [_, seq_len] = x.dims();

        assert!(
            seq_len <= self.n_text_ctx,
            "Token sequence length {} must not exceed {}.",
            seq_len,
            self.n_text_ctx
        );

        let x = embedding(self.token_embedding.val(), x)
            + self
                .positional_embedding
                .val()
                .slice([0..seq_len])
                .unsqueeze::<3>();

        //let mask = attn_decoder_mask(seq_len);

        let mut x = x;
        for block in &self.blocks {
            x = block.forward(x, xa.clone(), self.mask.val());
        }

        let x = self.ln.forward(x);
        x.matmul(self.token_embedding.val().transpose().unsqueeze::<3>())
    }
}

#[derive(Config)]
struct ResidualDecoderAttentionBlockConfig {
    n_state: usize,
    n_head: usize,
}

impl ResidualDecoderAttentionBlockConfig {
    fn init<B: Backend>(&self, device: &B::Device) -> ResidualDecoderAttentionBlock<B> {
        let attn = MultiHeadSelfAttentionConfig::new(self.n_state, self.n_head).init(device);
        let attn_ln = LayerNormConfig::new(self.n_state).init(device);

        let cross_attn = MultiHeadCrossAttentionConfig::new(self.n_state, self.n_head).init(device);
        let cross_attn_ln = LayerNormConfig::new(self.n_state).init(device);

        let mlp = MLPConfig::new(self.n_state).init(device);
        let mlp_ln = LayerNormConfig::new(self.n_state).init(device);

        ResidualDecoderAttentionBlock {
            attn,
            attn_ln,
            cross_attn,
            cross_attn_ln,
            mlp,
            mlp_ln,
        }
    }
}

#[derive(Module, Debug)]
struct ResidualDecoderAttentionBlock<B: Backend> {
    attn: MultiHeadSelfAttention<B>,
    attn_ln: LayerNorm<B>,
    cross_attn: MultiHeadCrossAttention<B>,
    cross_attn_ln: LayerNorm<B>,
    mlp: Mlp<B>,
    mlp_ln: LayerNorm<B>,
}

impl<B: Backend> ResidualDecoderAttentionBlock<B> {
    fn forward(&self, x: Tensor<B, 3>, xa: Tensor<B, 3>, mask: Tensor<B, 2>) -> Tensor<B, 3> {
        let x = x.clone() + self.attn.forward(self.attn_ln.forward(x), Some(mask));
        let x = x.clone() + self.cross_attn.forward(self.cross_attn_ln.forward(x), xa);
        x.clone() + self.mlp.forward(self.mlp_ln.forward(x))
    }
}

#[derive(Config)]
struct ResidualEncoderAttentionBlockConfig {
    n_state: usize,
    n_head: usize,
}

impl ResidualEncoderAttentionBlockConfig {
    fn init<B: Backend>(&self, device: &B::Device) -> ResidualEncoderAttentionBlock<B> {
        let attn = MultiHeadSelfAttentionConfig::new(self.n_state, self.n_head).init(device);
        let attn_ln = LayerNormConfig::new(self.n_state).init(device);
        let mlp = MLPConfig::new(self.n_state).init(device);
        let mlp_ln = LayerNormConfig::new(self.n_state).init(device);

        ResidualEncoderAttentionBlock {
            attn,
            attn_ln,
            mlp,
            mlp_ln,
        }
    }
}

#[derive(Module, Debug)]
struct ResidualEncoderAttentionBlock<B: Backend> {
    attn: MultiHeadSelfAttention<B>,
    attn_ln: LayerNorm<B>,
    mlp: Mlp<B>,
    mlp_ln: LayerNorm<B>,
}

impl<B: Backend> ResidualEncoderAttentionBlock<B> {
    fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 3> {
        let x = x.clone() + self.attn.forward(self.attn_ln.forward(x), None);
        x.clone() + self.mlp.forward(self.mlp_ln.forward(x))
    }
}

#[derive(Config)]
struct MLPConfig {
    n_state: usize,
}

impl MLPConfig {
    fn init<B: Backend>(&self, device: &B::Device) -> Mlp<B> {
        let lin1 = LinearConfig::new(self.n_state, 4 * self.n_state).init(device);
        let gelu = Gelu::new();
        let lin2 = LinearConfig::new(4 * self.n_state, self.n_state).init(device);

        Mlp { lin1, gelu, lin2 }
    }
}

#[derive(Module, Debug)]
struct Mlp<B: Backend> {
    lin1: Linear<B>,
    gelu: Gelu,
    lin2: Linear<B>,
}

impl<B: Backend> Mlp<B> {
    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 3> {
        let x = self.lin1.forward(x);
        let x = self.gelu.forward(x);
        self.lin2.forward(x)
    }
}

#[derive(Config)]
struct MultiHeadSelfAttentionConfig {
    n_state: usize,
    n_head: usize,
}

impl MultiHeadSelfAttentionConfig {
    fn init<B: Backend>(&self, device: &B::Device) -> MultiHeadSelfAttention<B> {
        let n_head = self.n_head;
        let query = LinearConfig::new(self.n_state, self.n_state).init(device);
        let key = LinearConfig::new(self.n_state, self.n_state)
            .with_bias(false)
            .init(device);
        let value = LinearConfig::new(self.n_state, self.n_state).init(device);
        let out = LinearConfig::new(self.n_state, self.n_state).init(device);

        MultiHeadSelfAttention {
            n_head,
            query,
            key,
            value,
            out,
        }
    }
}

#[derive(Module, Debug)]
struct MultiHeadSelfAttention<B: Backend> {
    n_head: usize,
    query: Linear<B>,
    key: Linear<B>,
    value: Linear<B>,
    out: Linear<B>,
}

impl<B: Backend> MultiHeadSelfAttention<B> {
    pub fn forward(&self, x: Tensor<B, 3>, mask: Option<Tensor<B, 2>>) -> Tensor<B, 3> {
        let q = self.query.forward(x.clone());
        let k = self.key.forward(x.clone());
        let v = self.value.forward(x);

        let wv = qkv_attention(q, k, v, mask, self.n_head);

        self.out.forward(wv)
    }
}

#[derive(Config)]
struct MultiHeadCrossAttentionConfig {
    n_state: usize,
    n_head: usize,
}

impl MultiHeadCrossAttentionConfig {
    fn init<B: Backend>(&self, device: &B::Device) -> MultiHeadCrossAttention<B> {
        assert_eq!(self.n_state % self.n_head, 0, "State size {} must be a multiple of head size {}", self.n_state, self.n_head);

        let n_head = self.n_head;
        let query = LinearConfig::new(self.n_state, self.n_state).init(device);
        let key = LinearConfig::new(self.n_state, self.n_state)
            .with_bias(false)
            .init(device);
        let value = LinearConfig::new(self.n_state, self.n_state).init(device);
        let out = LinearConfig::new(self.n_state, self.n_state).init(device);

        MultiHeadCrossAttention {
            n_head,
            query,
            key,
            value,
            out,
        }
    }
}

#[derive(Module, Debug)]
struct MultiHeadCrossAttention<B: Backend> {
    n_head: usize,
    query: Linear<B>,
    key: Linear<B>,
    value: Linear<B>,
    out: Linear<B>,
}

impl<B: Backend> MultiHeadCrossAttention<B> {
    pub fn forward(&self, x: Tensor<B, 3>, xa: Tensor<B, 3>) -> Tensor<B, 3> {
        let q = self.query.forward(x);
        let k = self.key.forward(xa.clone());
        let v = self.value.forward(xa);

        let wv = qkv_attention(q, k, v, None, self.n_head);

        self.out.forward(wv)
    }
}

fn qkv_attention<B: Backend>(
    q: Tensor<B, 3>,
    k: Tensor<B, 3>,
    v: Tensor<B, 3>,
    mask: Option<Tensor<B, 2>>,
    n_head: usize,
) -> Tensor<B, 3> {
    let [n_batch, n_qctx, n_state] = q.dims();
    let [_, n_ctx, _] = k.dims();

    let scale = (n_state as f64 / n_head as f64).powf(-0.25);
    let n_hstate = n_state / n_head;

    let q = q
        .reshape([n_batch, n_qctx, n_head, n_hstate])
        .swap_dims(1, 2)
        * scale;
    let k = k
        .reshape([n_batch, n_ctx, n_head, n_hstate])
        .swap_dims(1, 2)
        .transpose()
        * scale;
    let v = v
        .reshape([n_batch, n_ctx, n_head, n_hstate])
        .swap_dims(1, 2);

    let qk = q.matmul(k);

    // apply mask
    let qk = if let Some(mask) = mask {
        qk + mask.slice([0..n_qctx, 0..n_ctx]).unsqueeze::<4>()
    } else {
        qk
    };

    // normalize value weightings
    let w = softmax(qk, 3);
    w.matmul(v).swap_dims(1, 2).flatten(2, 3)
}

fn attn_decoder_mask<B: Backend>(seq_length: usize, tensor_device_ref: &B::Device) -> Tensor<B, 2> {
    let mut mask = Tensor::<B, 2>::zeros([seq_length, seq_length], tensor_device_ref);

    for i in 0..(seq_length - 1) {
        let values = Tensor::<B, 2>::zeros([1, seq_length - (i + 1)], tensor_device_ref)
            .add_scalar(f32::NEG_INFINITY);
        mask = mask.slice_assign([i..i + 1, i + 1..seq_length], values);
    }

    mask
}
