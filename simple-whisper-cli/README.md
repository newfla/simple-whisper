# Simple Whisper CLI
A modest CLI for speech transcription

## Example 
`simple-whisper-cli transcribe recording.mp3 tiny_en en output.txt`

## Usage

```
Usage: simple-whisper-cli transcribe [OPTIONS] <INPUT_FILE> <MODEL> <LANGUAGE> <OUTPUT_FILE>

Arguments:
  <INPUT_FILE>   Audio file
  <MODEL>        Which whisper model to use
  <LANGUAGE>     Audio language
  <OUTPUT_FILE>  Output transcription file

Options:
      --ignore-cache  Ignore cached model files
  -v, --verbose       Verbose STDOUT
  -h, --help          Print help
```