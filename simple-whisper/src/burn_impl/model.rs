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
    /// The tiny model with only English support.
    #[strum(serialize = "tiny_en", to_string = "TinyEn - tiny_en")]
    TinyEn,
    /// The base model.
    #[default]
    #[strum(serialize = "base", to_string = "Base - base")]
    Base,
    /// The base model with only English support.
    #[strum(serialize = "base_en", to_string = "BaseEn - base_en")]
    BaseEn,
    /// The small model.
    #[strum(serialize = "small", to_string = "Small - small")]
    Small,
    /// The small model with only English support.
    #[strum(serialize = "small_en", to_string = "SmallEn - small_en")]
    SmallEn,
    /// The medium model.
    #[strum(serialize = "medium", to_string = "Medium - medium")]
    Medium,
    /// The medium model with only English support.
    #[strum(serialize = "medium_en", to_string = "MediumEn - medium_en")]
    MediumEn,
    /// The large model.
    #[strum(serialize = "large", to_string = "Large V1 - large")]
    Large,
    /// The large model v2.
    #[strum(serialize = "large_v2", to_string = "Large V2 - large_v2")]
    LargeV2,
    /// The large model v3.
    #[strum(serialize = "large_v3", to_string = "Large V3 - large_v3")]
    LargeV3,
}

impl Model {
    pub(crate) fn hf_coordinates(&self) -> HFCoordinates {
        let repo = Repo::with_revision(
            "newfla/simple-whisper".to_owned(),
            hf_hub::RepoType::Model,
            "main".to_owned(),
        );
        match self {
            Model::Tiny => HFCoordinates {
                repo,
                config: Some("tiny/tiny.cfg".to_owned()),
                model: "tiny/tiny.mpk".to_owned(),
                tokenizer: Some("tiny/tokenizer.json".to_owned()),
            },
            Model::TinyEn => HFCoordinates {
                repo,
                config: Some("tiny_en/tiny_en.cfg".to_owned()),
                model: "tiny_en/tiny_en.mpk".to_owned(),
                tokenizer: Some("tiny_en/tokenizer.json".to_owned()),
            },
            Model::Base => HFCoordinates {
                repo,
                config: Some("base/base.cfg".to_owned()),
                model: "base/base.mpk".to_owned(),
                tokenizer: Some("base/tokenizer.json".to_owned()),
            },
            Model::BaseEn => HFCoordinates {
                repo,
                config: Some("base_en/base_en.cfg".to_owned()),
                model: "base_en/base_en.mpk".to_owned(),
                tokenizer: Some("tiny/tokenizer.json".to_owned()),
            },
            Model::Small => HFCoordinates {
                repo,
                config: Some("small/small.cfg".to_owned()),
                model: "small/small.mpk".to_owned(),
                tokenizer: Some("small/tokenizer.json".to_owned()),
            },
            Model::SmallEn => HFCoordinates {
                repo,
                config: Some("small_en/small_en.cfg".to_owned()),
                model: "small_en/small_en.mpk".to_owned(),
                tokenizer: Some("small_en/tokenizer.json".to_owned()),
            },
            Model::Medium => HFCoordinates {
                repo,
                config: Some("medium/medium.cfg".to_owned()),
                model: "medium/medium.mpk".to_owned(),
                tokenizer: Some("medium/tokenizer.json".to_owned()),
            },
            Model::MediumEn => HFCoordinates {
                repo,
                config: Some("medium_en/medium_en.cfg".to_owned()),
                model: "medium_en/medium_en.mpk".to_owned(),
                tokenizer: Some("medium_en/tokenizer.json".to_owned()),
            },
            Model::Large => HFCoordinates {
                repo,
                config: Some("large-v1/large-v1.cfg".to_owned()),
                model: "large-v1/large-v1.mpk".to_owned(),
                tokenizer: Some("large-v1/tokenizer.json".to_owned()),
            },
            Model::LargeV2 => HFCoordinates {
                repo,
                config: Some("large-v2/large-v2.cfg".to_owned()),
                model: "large-v2/large-v2.mpk".to_owned(),
                tokenizer: Some("large-v2/tokenizer.json".to_owned()),
            },
            Model::LargeV3 => HFCoordinates {
                repo,
                config: Some("large-v3/large-v3.cfg".to_owned()),
                model: "large-v3/large-v3.mpk".to_owned(),
                tokenizer: Some("large-v3/tokenizer.json".to_owned()),
            },
        }
    }
}
