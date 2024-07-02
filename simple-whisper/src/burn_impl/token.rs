use strum::{Display, EnumIs};

use crate::Language;

#[derive(Debug, EnumIs, Display)]
pub enum SpecialToken {
    #[strum(to_string = "<|endoftext|>")]
    EndofText,
    #[strum(to_string = "<|startoftranscript|>")]
    StartofTranscript,
    #[strum(to_string = "<|transcribe|>")]
    Transcribe,
    #[strum(to_string = "<|notimestamps|>")]
    NoTimeStamps,
    #[strum(to_string = "<|{0}|>")]
    Language(Language),
}