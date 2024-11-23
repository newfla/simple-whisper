use hf_hub::Repo;
use strum::{Display, EnumIter, EnumString};

use crate::model::HFCoordinates;

/// OpenAI supported models
#[derive(Default, Clone, Debug, EnumIter, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Model {
    /// The tiny model.
    #[strum(serialize = "tiny", to_string = "Tiny - tiny")]
    Tiny,
    /// The tiny-q5_1 model.
    #[strum(serialize = "tiny-q5_1", to_string = "Tiny - tiny-q5_1")]
    TinyQ5_1,
    /// The tiny-q8_0 model.
    #[strum(serialize = "tiny-q8_0", to_string = "Tiny - tiny-q8_0")]
    TinyQ8_0,
    /// The tiny model with only English support.
    #[strum(serialize = "tiny_en", to_string = "TinyEn - tiny_en")]
    TinyEn,
    /// The tiny-q5_1 model with only English support.
    #[strum(serialize = "tiny_en-q5_1", to_string = "TinyEn - tiny_en-q5_1")]
    TinyEnQ5_1,
    /// The tiny-q8_0 model with only English support.
    #[strum(serialize = "tiny_en-q8_0", to_string = "Tiny - tiny_en-q8_0")]
    TinyEnQ8_0,
    /// The base model.
    #[default]
    #[strum(serialize = "base", to_string = "Base - base")]
    Base,
    /// The base-q5_1 model.
    #[strum(serialize = "base-q5_1", to_string = "Base - base-q5_1")]
    BaseQ5_1,
    /// The base-q8_0 model.
    #[strum(serialize = "base-q8_0", to_string = "Base - base-q8_0")]
    BaseQ8_0,
    /// The base model with only English support.
    #[strum(serialize = "base_en", to_string = "BaseEn - base_en")]
    BaseEn,
    /// The base-q5_1 model with only English support.
    #[strum(serialize = "base_en-q5_1", to_string = "BaseEn -base_en-q5_1")]
    BaseEnQ5_1,
    /// The base-q8_0 model with only English support.
    #[strum(serialize = "base_en-q8_0", to_string = "BaseEn - base_en-q8_0")]
    BaseEnQ8_0,
    /// The small model.
    #[strum(serialize = "small", to_string = "Small - small")]
    Small,
    /// The small-q5_1 model.
    #[strum(serialize = "small-q5_1", to_string = "Small - small-q5_1")]
    SmallQ5_1,
    /// The small-q8_0 model.
    #[strum(serialize = "small-q8_0", to_string = "Small - small-q8_0")]
    SmallQ8_0,
    /// The small model with only English support.
    #[strum(serialize = "small_en", to_string = "SmallEn - small_en")]
    SmallEn,
    /// The small-q5_1 model with only English support.
    #[strum(serialize = "small_en-q5_1", to_string = "SmallEn - small_en-q5_1")]
    SmallEnQ5_1,
    /// The small-q8_0 model with only English support.
    #[strum(serialize = "small_en-q8_0", to_string = "SmallEn - small_en-q8_0")]
    SmallEnQ8_0,
    /// The medium model.
    #[strum(serialize = "medium", to_string = "Medium - medium")]
    Medium,
    /// The medium-q5_0 model.
    #[strum(serialize = "medium-q5_0", to_string = "Medium - medium-q5_0")]
    MediumQ5_0,
    /// The medium-q8_0 model.
    #[strum(serialize = "medium-q8_0", to_string = "Medium - medium-q8_0")]
    MediumQ8_0,
    /// The medium model with only English support.
    #[strum(serialize = "medium_en", to_string = "MediumEn - medium_en")]
    MediumEn,
    /// The medium-q5_0 model with only English support.
    #[strum(serialize = "medium_en-q5_0	", to_string = "MediumEn - medium_en-q5_0")]
    MediumEnQ5_0,
    /// The medium-q8_0 model with only English support.
    #[strum(serialize = "medium_en-q8_0", to_string = "MediumEn - medium_en-q8_0")]
    MediumEnQ8_0,
    /// The large model.
    #[strum(serialize = "large", to_string = "Large V1 - large")]
    Large,
    /// The large model v2.
    #[strum(serialize = "large_v2", to_string = "Large V2 - large_v2")]
    LargeV2,
    #[strum(serialize = "large_v2-q5_0", to_string = "Large V2 - large_v2-q5_0")]
    LargeV2Q5_0,
    #[strum(serialize = "large_v2-q8_0", to_string = "Large V2 - large_v2-q8_0")]
    LargeV2Q8_0,
    /// The large model v3.
    #[strum(serialize = "large_v3", to_string = "Large V3 - large_v3")]
    LargeV3,
    /// The large_v3-q5_0 model v3.
    #[strum(serialize = "large_v3-q5_0", to_string = "Large V3 - large_v3-q5_0")]
    LargeV3Q5_0,
    /// The large model v3 turbo.
    #[strum(
        serialize = "large_v3_turbo",
        to_string = "Large V3 Turbo - large_v3_turbo"
    )]
    LargeV3Turbo,
    /// The large_v3_turbo-q5_0 model v3 turbo.
    #[strum(
        serialize = "large_v3_turbo-q5_0",
        to_string = "Large V3 Turbo - large_v3_turbo-q5_0"
    )]
    LargeV3TurboQ5_0,
    /// The large_v3_turbo-q8_0 model v3 turbo.
    #[strum(
        serialize = "large_v3_turbo-q8_0",
        to_string = "Large V3 Turbo - large_v3_turbo-q8_0"
    )]
    LargeV3TurboQ8_0,
}

impl Model {
    pub(crate) fn hf_coordinates(&self) -> HFCoordinates {
        let repo = Repo::with_revision(
            "ggerganov/whisper.cpp".to_owned(),
            hf_hub::RepoType::Model,
            "main".to_owned(),
        );
        match self {
            Model::Tiny => HFCoordinates {
                repo,
                config: None,
                model: "ggml-tiny.bin".to_owned(),
                tokenizer: None,
            },
            Model::TinyEn => HFCoordinates {
                repo,
                config: None,
                model: "ggml-tiny.en.bin".to_owned(),
                tokenizer: None,
            },
            Model::Base => HFCoordinates {
                repo,
                config: None,
                model: "ggml-base.bin".to_owned(),
                tokenizer: None,
            },
            Model::BaseEn => HFCoordinates {
                repo,
                config: None,
                model: "ggml-base.en.bin".to_owned(),
                tokenizer: None,
            },
            Model::Small => HFCoordinates {
                repo,
                config: None,
                model: "ggml-small.bin".to_owned(),
                tokenizer: None,
            },
            Model::SmallEn => HFCoordinates {
                repo,
                config: None,
                model: "ggml-small.en.bin".to_owned(),
                tokenizer: None,
            },
            Model::Medium => HFCoordinates {
                repo,
                config: None,
                model: "ggml-medium.bin".to_owned(),
                tokenizer: None,
            },
            Model::MediumEn => HFCoordinates {
                repo,
                config: None,
                model: "ggml-medium.en.bin".to_owned(),
                tokenizer: None,
            },
            Model::Large => HFCoordinates {
                repo,
                config: None,
                model: "ggml-large-v1.bin".to_owned(),
                tokenizer: None,
            },
            Model::LargeV2 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-large-v2.bin".to_owned(),
                tokenizer: None,
            },
            Model::LargeV3 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-large-v3.bin".to_owned(),
                tokenizer: None,
            },
            Model::TinyQ5_1 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-tiny-q5_1.bin".to_owned(),
                tokenizer: None,
            },
            Model::TinyQ8_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-tiny-q8_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::TinyEnQ5_1 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-tiny.en-q5_1.bin".to_owned(),
                tokenizer: None,
            },
            Model::TinyEnQ8_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-tiny.en-q8_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::BaseQ5_1 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-base-q5_1.bin".to_owned(),
                tokenizer: None,
            },
            Model::BaseQ8_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-base-q8_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::BaseEnQ5_1 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-base.en-q5_1.bin".to_owned(),
                tokenizer: None,
            },
            Model::BaseEnQ8_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-base.en-q8_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::SmallQ5_1 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-small-q5_1.bin".to_owned(),
                tokenizer: None,
            },
            Model::SmallQ8_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-small-q8_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::SmallEnQ5_1 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-small.en-q5_1.bin".to_owned(),
                tokenizer: None,
            },
            Model::SmallEnQ8_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-small.en-q8_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::MediumQ5_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-medium-q5_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::MediumQ8_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-medium-q8_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::MediumEnQ5_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-medium.en-q5_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::MediumEnQ8_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-medium.en-q8_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::LargeV2Q5_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-large-v2-q5_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::LargeV2Q8_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-large-v2-q8_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::LargeV3Q5_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-large-v3-q5_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::LargeV3Turbo => HFCoordinates {
                repo,
                config: None,
                model: "ggml-large-v3-turbo.bin".to_owned(),
                tokenizer: None,
            },
            Model::LargeV3TurboQ5_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-large-v3-turbo-q5_0.bin".to_owned(),
                tokenizer: None,
            },
            Model::LargeV3TurboQ8_0 => HFCoordinates {
                repo,
                config: None,
                model: "ggml-large-v3-turbo-q8_0.bin".to_owned(),
                tokenizer: None,
            },
        }
    }
}
