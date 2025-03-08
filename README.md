# Simple Whisper

Transcription library/cli/server based on [OpenAI Whisper](https://github.com/openai/whisper) model written using [whisper-rs](https://github.com/tazz4843/whisper-rs).

## What is included?

- [Simple Whisper lib](./simple-whisper/): Implements the Whisper model via:
  - [whisper.cpp Backend](https://github.com/tazz4843/whisper-rs). Weights are automatically downloaded from [Hugging Face repo](https://huggingface.co/ggerganov/whisper.cpp). 
  - Supported codec: flac, vorbis, wav, mp3

- [Simple Whisper cli](./simple-whisper-cli/): CLI application useful to transcribe audio file. For more information see the [README.md](./simple-whisper-cli/README.md).

- [Simple Whisper server](./simple-whisper-server/): Websocket server that transcribe uploaded files.

## Goals
- Show how malleable RUST is, scaling from server to GPU code.
- Support a high variety of platforms.
- Fast enough on every platform.

## No Goals
- It is **NOT** intended to be the fastest/accurate Whisper implementation.

## Credits
The project was inspired by:
- Candle implementation: [rwhisper](https://github.com/floneum/floneum/tree/main/models/rwhisper).