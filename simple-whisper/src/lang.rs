use strum::{Display, EnumIs, EnumIter, EnumString};

#[derive(Default, Clone, Debug, EnumIs, EnumIter, EnumString, Display)]
pub enum Language {
    #[default]
    #[strum(serialize = "en", to_string = "English - en")]
    English,
    #[strum(serialize = "zh", to_string = "Chinese - zh")]
    Chinese,
    #[strum(serialize = "de", to_string = "German - de")]
    German,
    #[strum(serialize = "es", to_string = "Spanish - es")]
    Spanish,
    #[strum(serialize = "ru", to_string = "Russian - ru")]
    Russian,
    #[strum(serialize = "ko", to_string = "Korean - ko")]
    Korean,
    #[strum(serialize = "fr", to_string = "French - fr")]
    French,
    #[strum(serialize = "ja", to_string = "Japanese - ja")]
    Japanese,
    #[strum(serialize = "pt", to_string = "Portuguese - pt")]
    Portuguese,
    #[strum(serialize = "tr", to_string = "Turkish - tr")]
    Turkish,
    #[strum(serialize = "pl", to_string = "Polish - pl")]
    Polish,
    #[strum(serialize = "ca", to_string = "Catalan - ca")]
    Catalan,
    #[strum(serialize = "nl", to_string = "Dutch - nl")]
    Dutch,
    #[strum(serialize = "ar", to_string = "Arabic - ar")]
    Arabic,
    #[strum(serialize = "sv", to_string = "Swedish - sv")]
    Swedish,
    #[strum(serialize = "it", to_string = "Italian - it")]
    Italian,
    #[strum(serialize = "id", to_string = "Indonesian - id")]
    Indonesian,
    #[strum(serialize = "hi", to_string = "Hindi - hi")]
    Hindi,
    #[strum(serialize = "fi", to_string = "Finnish - fi")]
    Finnish,
    #[strum(serialize = "vi", to_string = "Vietnamese - vi")]
    Vietnamese,
    #[strum(serialize = "he", to_string = "Hebrew - he")]
    Hebrew,
    #[strum(serialize = "uk", to_string = "Ukrainian - uk")]
    Ukrainian,
    #[strum(serialize = "el", to_string = "Greek - el")]
    Greek,
    #[strum(serialize = "ms", to_string = "Malay - ms")]
    Malay,
    #[strum(serialize = "cs", to_string = "Czech - cs")]
    Czech,
    #[strum(serialize = "ro", to_string = "Romanian - ro")]
    Romanian,
    #[strum(serialize = "da", to_string = "Danish - da")]
    Danish,
    #[strum(serialize = "hu", to_string = "Hungarian - hu")]
    Hungarian,
    #[strum(serialize = "ta", to_string = "Tamil - ta")]
    Tamil,
    #[strum(serialize = "no", to_string = "Norwegian - no")]
    Norwegian,
    #[strum(serialize = "th", to_string = "Thai - th")]
    Thai,
    #[strum(serialize = "ur", to_string = "Urdu - ur")]
    Urdu,
    #[strum(serialize = "hr", to_string = "Croatian - hr")]
    Croatian,
    #[strum(serialize = "bg", to_string = "Bulgarian - bg")]
    Bulgarian,
    #[strum(serialize = "lt", to_string = "Lithuanian - lt")]
    Lithuanian,
    #[strum(serialize = "la", to_string = "Latin - la")]
    Latin,
    #[strum(serialize = "mi", to_string = "Maori - mi")]
    Maori,
    #[strum(serialize = "ml", to_string = "Malayalam - ml")]
    Malayalam,
    #[strum(serialize = "cy", to_string = "Welsh - cy")]
    Welsh,
    #[strum(serialize = "sk", to_string = "Slovak - sk")]
    Slovak,
    #[strum(serialize = "te", to_string = "Telugu - te")]
    Telugu,
    #[strum(serialize = "fa", to_string = "Persian - fa")]
    Persian,
    #[strum(serialize = "lv", to_string = "Latvian - lv")]
    Latvian,
    #[strum(serialize = "bn", to_string = "Bengali - bn")]
    Bengali,
    #[strum(serialize = "sr", to_string = "Serbian - sr")]
    Serbian,
    #[strum(serialize = "az", to_string = "Azerbaijani - az")]
    Azerbaijani,
    #[strum(serialize = "sl", to_string = "Slovenian - sl")]
    Slovenian,
    #[strum(serialize = "kn", to_string = "Kannada - kn")]
    Kannada,
    #[strum(serialize = "et", to_string = "Estonian - et")]
    Estonian,
    #[strum(serialize = "mk", to_string = "Macedonian - mk")]
    Macedonian,
    #[strum(serialize = "br", to_string = "Breton - br")]
    Breton,
    #[strum(serialize = "eu", to_string = "Basque - eu")]
    Basque,
    #[strum(serialize = "is", to_string = "Icelandic - is")]
    Icelandic,
    #[strum(serialize = "hy", to_string = "Armenian - hy")]
    Armenian,
    #[strum(serialize = "ne", to_string = "Nepali - ne")]
    Nepali,
    #[strum(serialize = "mn", to_string = "Mongolian - mn")]
    Mongolian,
    #[strum(serialize = "bs", to_string = "Bosnian - bs")]
    Bosnian,
    #[strum(serialize = "kk", to_string = "Kazakh - kk")]
    Kazakh,
    #[strum(serialize = "sq", to_string = "Albanian - sq")]
    Albanian,
    #[strum(serialize = "sw", to_string = "Swahili - sw")]
    Swahili,
    #[strum(serialize = "gl", to_string = "Galician - gl")]
    Galician,
    #[strum(serialize = "mr", to_string = "Marathi - mr")]
    Marathi,
    #[strum(serialize = "pa", to_string = "Punjabi - pa")]
    Punjabi,
    #[strum(serialize = "si", to_string = "Sinhala - si")]
    Sinhala,
    #[strum(serialize = "km", to_string = "Khmer - km")]
    Khmer,
    #[strum(serialize = "sn", to_string = "Shona - sn")]
    Shona,
    #[strum(serialize = "yo", to_string = "Yoruba - yo")]
    Yoruba,
    #[strum(serialize = "so", to_string = "Somali - so")]
    Somali,
    #[strum(serialize = "af", to_string = "Afrikaans - af")]
    Afrikaans,
    #[strum(serialize = "oc", to_string = "Occitan - oc")]
    Occitan,
    #[strum(serialize = "ka", to_string = "Georgian - ka")]
    Georgian,
    #[strum(serialize = "be", to_string = "Belarusian - be")]
    Belarusian,
    #[strum(serialize = "tg", to_string = "Tajik - tg")]
    Tajik,
    #[strum(serialize = "sd", to_string = "Sindhi - sd")]
    Sindhi,
    #[strum(serialize = "gu", to_string = "Gujarati - gu")]
    Gujarati,
    #[strum(serialize = "am", to_string = "Amharic - am")]
    Amharic,
    #[strum(serialize = "yi", to_string = "Yiddish - yi")]
    Yiddish,
    #[strum(serialize = "lo", to_string = "Lao - lo")]
    Lao,
    #[strum(serialize = "uz", to_string = "Uzbek - uz")]
    Uzbek,
    #[strum(serialize = "fo", to_string = "Faroese - fo")]
    Faroese,
    #[strum(serialize = "ht", to_string = "HaitianCreole - ht")]
    HaitianCreole,
    #[strum(serialize = "ps", to_string = "Pashto - ps")]
    Pashto,
    #[strum(serialize = "tk", to_string = "Turkmen - tk")]
    Turkmen,
    #[strum(serialize = "nn", to_string = "Nynorsk - nn")]
    Nynorsk,
    #[strum(serialize = "mt", to_string = "Maltese - mt")]
    Maltese,
    #[strum(serialize = "sa", to_string = "Sanskrit - sa")]
    Sanskrit,
    #[strum(serialize = "lb", to_string = "Luxembourgish - lb")]
    Luxembourgish,
    #[strum(serialize = "my", to_string = "Myanmar - my")]
    Myanmar,
    #[strum(serialize = "bo", to_string = "Tibetan - bo")]
    Tibetan,
    #[strum(serialize = "tl", to_string = "Tagalog - tl")]
    Tagalog,
    #[strum(serialize = "mg", to_string = "Malagasy - mg")]
    Malagasy,
    #[strum(serialize = "as", to_string = "Assamese - as")]
    Assamese,
    #[strum(serialize = "tt", to_string = "Tatar - tt")]
    Tatar,
    #[strum(serialize = "haw", to_string = "Hawaiian - haw")]
    Hawaiian,
    #[strum(serialize = "ln", to_string = "Lingala - ln")]
    Lingala,
    #[strum(serialize = "ha", to_string = "Hausa - ha")]
    Hausa,
    #[strum(serialize = "ba", to_string = "Bashkir - ba")]
    Bashkir,
    #[strum(serialize = "jw", to_string = "Javanese - jw")]
    Javanese,
    #[strum(serialize = "su", to_string = "Sundanese - su")]
    Sundanese,
}

/*impl Language {
    fn code(&self) -> &str {
        match self {
            Self::English => "en",
            Self::Chinese => "zh",
            Self::German => "de",
            Self::Spanish => "es",
            Self::Russian => "ru",
            Self::Korean => "ko",
            Self::French => "fr",
            Self::Japanese => "ja",
            Self::Portuguese => "pt",
            Self::Turkish => "tr",
            Self::Polish => "pl",
            Self::Catalan => "ca",
            Self::Dutch => "nl",
            Self::Arabic => "ar",
            Self::Swedish => "sv",
            Self::Italian => "it",
            Self::Indonesian => "id",
            Self::Hindi => "hi",
            Self::Finnish => "fi",
            Self::Vietnamese => "vi",
            Self::Hebrew => "he",
            Self::Ukrainian => "uk",
            Self::Greek => "el",
            Self::Malay => "ms",
            Self::Czech => "cs",
            Self::Romanian => "ro",
            Self::Danish => "da",
            Self::Hungarian => "hu",
            Self::Tamil => "ta",
            Self::Norwegian => "no",
            Self::Thai => "th",
            Self::Urdu => "ur",
            Self::Croatian => "hr",
            Self::Bulgarian => "bg",
            Self::Lithuanian => "lt",
            Self::Latin => "la",
            Self::Maori => "mi",
            Self::Malayalam => "ml",
            Self::Welsh => "cy",
            Self::Slovak => "sk",
            Self::Telugu => "te",
            Self::Persian => "fa",
            Self::Latvian => "lv",
            Self::Bengali => "bn",
            Self::Serbian => "sr",
            Self::Azerbaijani => "az",
            Self::Slovenian => "sl",
            Self::Kannada => "kn",
            Self::Estonian => "et",
            Self::Macedonian => "mk",
            Self::Breton => "br",
            Self::Basque => "eu",
            Self::Icelandic => "is",
            Self::Armenian => "hy",
            Self::Nepali => "ne",
            Self::Mongolian => "mn",
            Self::Bosnian => "bs",
            Self::Kazakh => "kk",
            Self::Albanian => "sq",
            Self::Swahili => "sw",
            Self::Galician => "gl",
            Self::Marathi => "mr",
            Self::Punjabi => "pa",
            Self::Sinhala => "si",
            Self::Khmer => "km",
            Self::Shona => "sn",
            Self::Yoruba => "yo",
            Self::Somali => "so",
            Self::Afrikaans => "af",
            Self::Occitan => "oc",
            Self::Georgian => "ka",
            Self::Belarusian => "be",
            Self::Tajik => "tg",
            Self::Sindhi => "sd",
            Self::Gujarati => "gu",
            Self::Amharic => "am",
            Self::Yiddish => "yi",
            Self::Lao => "lo",
            Self::Uzbek => "uz",
            Self::Faroese => "fo",
            Self::HaitianCreole => "ht",
            Self::Pashto => "ps",
            Self::Turkmen => "tk",
            Self::Nynorsk => "nn",
            Self::Maltese => "mt",
            Self::Sanskrit => "sa",
            Self::Luxembourgish => "lb",
            Self::Myanmar => "my",
            Self::Tibetan => "bo",
            Self::Tagalog => "tl",
            Self::Malagasy => "mg",
            Self::Assamese => "as",
            Self::Tatar => "tt",
            Self::Hawaiian => "haw",
            Self::Lingala => "ln",
            Self::Hausa => "ha",
            Self::Bashkir => "ba",
            Self::Javanese => "jw",
            Self::Sundanese => "su",
        }
    }
}*/
/*
impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value {
            "en" => Self::English,
            "zh" => Self::Chinese,
            "de" => Self::German,
            "es" => Self::Spanish,
            "ru" => Self::Russian,
            "ko" => Self::Korean,
            "fr" => Self::French,
            "ja" => Self::Japanese,
            "pt" => Self::Portuguese,
            "tr" => Self::Turkish,
            "pl" => Self::Polish,
            "ca" => Self::Catalan,
            "nl" => Self::Dutch,
            "ar" => Self::Arabic,
            "sv" => Self::Swedish,
            "it" => Self::Italian,
            "id" => Self::Indonesian,
            "hi" => Self::Hindi,
            "fi" => Self::Finnish,
            "vi" => Self::Vietnamese,
            "he" => Self::Hebrew,
            "uk" => Self::Ukrainian,
            "el" => Self::Greek,
            "ms" => Self::Malay,
            "cs" => Self::Czech,
            "ro" => Self::Romanian,
            "da" => Self::Danish,
            "hu" => Self::Hungarian,
            "ta" => Self::Tamil,
            "no" => Self::Norwegian,
            "th" => Self::Thai,
            "ur" => Self::Urdu,
            "hr" => Self::Croatian,
            "bg" => Self::Bulgarian,
            "lt" => Self::Lithuanian,
            "la" => Self::Latin,
            "mi" => Self::Maori,
            "ml" => Self::Malayalam,
            "cy" => Self::Welsh,
            "sk" => Self::Slovak,
            "te" => Self::Telugu,
            "fa" => Self::Persian,
            "lv" => Self::Latvian,
            "bn" => Self::Bengali,
            "sr" => Self::Serbian,
            "az" => Self::Azerbaijani,
            "sl" => Self::Slovenian,
            "kn" => Self::Kannada,
            "et" => Self::Estonian,
            "mk" => Self::Macedonian,
            "br" => Self::Breton,
            "eu" => Self::Basque,
            "is" => Self::Icelandic,
            "hy" => Self::Armenian,
            "ne" => Self::Nepali,
            "mn" => Self::Mongolian,
            "bs" => Self::Bosnian,
            "kk" => Self::Kazakh,
            "sq" => Self::Albanian,
            "sw" => Self::Swahili,
            "gl" => Self::Galician,
            "mr" => Self::Marathi,
            "pa" => Self::Punjabi,
            "si" => Self::Sinhala,
            "km" => Self::Khmer,
            "sn" => Self::Shona,
            "yo" => Self::Yoruba,
            "so" => Self::Somali,
            "af" => Self::Afrikaans,
            "oc" => Self::Occitan,
            "ka" => Self::Georgian,
            "be" => Self::Belarusian,
            "tg" => Self::Tajik,
            "sd" => Self::Sindhi,
            "gu" => Self::Gujarati,
            "am" => Self::Amharic,
            "yi" => Self::Yiddish,
            "lo" => Self::Lao,
            "uz" => Self::Uzbek,
            "fo" => Self::Faroese,
            "ht" => Self::HaitianCreole,
            "ps" => Self::Pashto,
            "tk" => Self::Turkmen,
            "nn" => Self::Nynorsk,
            "mt" => Self::Maltese,
            "sa" => Self::Sanskrit,
            "lb" => Self::Luxembourgish,
            "my" => Self::Myanmar,
            "bo" => Self::Tibetan,
            "tl" => Self::Tagalog,
            "mg" => Self::Malagasy,
            "as" => Self::Assamese,
            "tt" => Self::Tatar,
            "haw" => Self::Hawaiian,
            "ln" => Self::Lingala,
            "ha" => Self::Hausa,
            "ba" => Self::Bashkir,
            "jw" => Self::Javanese,
            "su" => Self::Sundanese,
            _ => Language::English,
        }
    }
}
*/
/*
impl Display for Language {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?} {}", self, self.code())
    }
}
*/
