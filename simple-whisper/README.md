# Simple Whisper 
Implements the Whisper model via the [Burn WGPU backend](https://github.com/tracel-ai/burn/blob/main/crates/burn-wgpu/README.md) or [whisper-rs](https://github.com/tazz4843/whisper-rs).

Burn implmentation is based over [sudomonikers/whisper-burn](https://github.com/sudomonikers/whisper-burn), provides abstraction over different models and languages. 

Weights are automatically downloaded from Hugging Face.

## Feature flags
 - `burn_vulkan` =  **default** enables the Burn WGPU backend
 - `whisper_cpp_vulkan` = enables the alternative whisper.cpp backed using Vulkan shaders
## Other resources
See [newfla/simple-whisper](https://github.com/newfla/simple-whisper) for prebuilt cli & server binaries
