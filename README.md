# Simple Whisper

Transcription library/cli/server base on [OpenAI Whisper](https://github.com/openai/whisper) model written using [Burn framework](https://github.com/tracel-ai/burn)🔥 or [whisper-rs](https://github.com/tazz4843/whisper-rs).

## What is included?

- [Simple Whisper lib](./simple-whisper/): Implements the Whisper model via:
  1. [Burn WGPU backend](https://github.com/tracel-ai/burn/blob/main/crates/burn-wgpu/README.md). Based over [sudomonikers/whisper-burn](https://github.com/sudomonikers/whisper-burn), provides abstraction over different models and languages. Weights are automatically downloaded from [Hugging Face repo](https://huggingface.co/newfla/simple-whisper). 
  2. [whisper.cpp Vulkan Backend](https://github.com/tazz4843/whisper-rs). Weights are automatically downloaded from [Hugging Face repo](https://huggingface.co/ggerganov/whisper.cpp). 
  - Supported codec: flac, vorbis, wav, mp3

- [Simple Whisper cli](./simple-whisper-cli/): CLI application useful to transcribe audio file. For more information see the [README.md](./simple-whisper-cli/README.md).

- [Simple Whisper server](./simple-whisper-server/): Websocket server that transcribe uploaded files.

## Goals
- Show how malleable RUST is, scaling from server to GPU code.
- 0 build prerequisites.
- Support a high variety of platforms.
- Fast enough on every platform.

## No Goals
- It is **NOT** intended to be the fastest/accurate Whisper implementation.
- **NOT production ready** 

## Credits
The project was inspired by:
- Previous whisper Burn implementations: [sudomonikers/whisper-burn](https://github.com/sudomonikers/whisper-burn) and [Gadersd/whisper-burn](https://github.com/Gadersd/whisper-burn).
- Candle implementation: [rwhisper](https://github.com/floneum/floneum/tree/main/models/rwhisper).