use strum::{Display, EnumIs, EnumIter, EnumMessage, EnumString};

/// Languages supported by the tokenizer
#[derive(Default, Clone, Copy, Debug, EnumIs, EnumIter, EnumString, Display, EnumMessage)]
pub enum Language {
    #[default]
    #[strum(serialize = "en", message = "English - en")]
    English,
    #[strum(serialize = "zh", message = "Chinese - zh")]
    Chinese,
    #[strum(serialize = "de", message = "German - de")]
    German,
    #[strum(serialize = "es", message = "Spanish - es")]
    Spanish,
    #[strum(serialize = "ru", message = "Russian - ru")]
    Russian,
    #[strum(serialize = "ko", message = "Korean - ko")]
    Korean,
    #[strum(serialize = "fr", message = "French - fr")]
    French,
    #[strum(serialize = "ja", message = "Japanese - ja")]
    Japanese,
    #[strum(serialize = "pt", message = "Portuguese - pt")]
    Portuguese,
    #[strum(serialize = "tr", message = "Turkish - tr")]
    Turkish,
    #[strum(serialize = "pl", message = "Polish - pl")]
    Polish,
    #[strum(serialize = "ca", message = "Catalan - ca")]
    Catalan,
    #[strum(serialize = "nl", message = "Dutch - nl")]
    Dutch,
    #[strum(serialize = "ar", message = "Arabic - ar")]
    Arabic,
    #[strum(serialize = "sv", message = "Swedish - sv")]
    Swedish,
    #[strum(serialize = "it", message = "Italian - it")]
    Italian,
    #[strum(serialize = "id", message = "Indonesian - id")]
    Indonesian,
    #[strum(serialize = "hi", message = "Hindi - hi")]
    Hindi,
    #[strum(serialize = "fi", message = "Finnish - fi")]
    Finnish,
    #[strum(serialize = "vi", message = "Vietnamese - vi")]
    Vietnamese,
    #[strum(serialize = "he", message = "Hebrew - he")]
    Hebrew,
    #[strum(serialize = "uk", message = "Ukrainian - uk")]
    Ukrainian,
    #[strum(serialize = "el", message = "Greek - el")]
    Greek,
    #[strum(serialize = "ms", message = "Malay - ms")]
    Malay,
    #[strum(serialize = "cs", message = "Czech - cs")]
    Czech,
    #[strum(serialize = "ro", message = "Romanian - ro")]
    Romanian,
    #[strum(serialize = "da", message = "Danish - da")]
    Danish,
    #[strum(serialize = "hu", message = "Hungarian - hu")]
    Hungarian,
    #[strum(serialize = "ta", message = "Tamil - ta")]
    Tamil,
    #[strum(serialize = "no", message = "Norwegian - no")]
    Norwegian,
    #[strum(serialize = "th", message = "Thai - th")]
    Thai,
    #[strum(serialize = "ur", message = "Urdu - ur")]
    Urdu,
    #[strum(serialize = "hr", message = "Croatian - hr")]
    Croatian,
    #[strum(serialize = "bg", message = "Bulgarian - bg")]
    Bulgarian,
    #[strum(serialize = "lt", message = "Lithuanian - lt")]
    Lithuanian,
    #[strum(serialize = "la", message = "Latin - la")]
    Latin,
    #[strum(serialize = "mi", message = "Maori - mi")]
    Maori,
    #[strum(serialize = "ml", message = "Malayalam - ml")]
    Malayalam,
    #[strum(serialize = "cy", message = "Welsh - cy")]
    Welsh,
    #[strum(serialize = "sk", message = "Slovak - sk")]
    Slovak,
    #[strum(serialize = "te", message = "Telugu - te")]
    Telugu,
    #[strum(serialize = "fa", message = "Persian - fa")]
    Persian,
    #[strum(serialize = "lv", message = "Latvian - lv")]
    Latvian,
    #[strum(serialize = "bn", message = "Bengali - bn")]
    Bengali,
    #[strum(serialize = "sr", message = "Serbian - sr")]
    Serbian,
    #[strum(serialize = "az", message = "Azerbaijani - az")]
    Azerbaijani,
    #[strum(serialize = "sl", message = "Slovenian - sl")]
    Slovenian,
    #[strum(serialize = "kn", message = "Kannada - kn")]
    Kannada,
    #[strum(serialize = "et", message = "Estonian - et")]
    Estonian,
    #[strum(serialize = "mk", message = "Macedonian - mk")]
    Macedonian,
    #[strum(serialize = "br", message = "Breton - br")]
    Breton,
    #[strum(serialize = "eu", message = "Basque - eu")]
    Basque,
    #[strum(serialize = "is", message = "Icelandic - is")]
    Icelandic,
    #[strum(serialize = "hy", message = "Armenian - hy")]
    Armenian,
    #[strum(serialize = "ne", message = "Nepali - ne")]
    Nepali,
    #[strum(serialize = "mn", message = "Mongolian - mn")]
    Mongolian,
    #[strum(serialize = "bs", message = "Bosnian - bs")]
    Bosnian,
    #[strum(serialize = "kk", message = "Kazakh - kk")]
    Kazakh,
    #[strum(serialize = "sq", message = "Albanian - sq")]
    Albanian,
    #[strum(serialize = "sw", message = "Swahili - sw")]
    Swahili,
    #[strum(serialize = "gl", message = "Galician - gl")]
    Galician,
    #[strum(serialize = "mr", message = "Marathi - mr")]
    Marathi,
    #[strum(serialize = "pa", message = "Punjabi - pa")]
    Punjabi,
    #[strum(serialize = "si", message = "Sinhala - si")]
    Sinhala,
    #[strum(serialize = "km", message = "Khmer - km")]
    Khmer,
    #[strum(serialize = "sn", message = "Shona - sn")]
    Shona,
    #[strum(serialize = "yo", message = "Yoruba - yo")]
    Yoruba,
    #[strum(serialize = "so", message = "Somali - so")]
    Somali,
    #[strum(serialize = "af", message = "Afrikaans - af")]
    Afrikaans,
    #[strum(serialize = "oc", message = "Occitan - oc")]
    Occitan,
    #[strum(serialize = "ka", message = "Georgian - ka")]
    Georgian,
    #[strum(serialize = "be", message = "Belarusian - be")]
    Belarusian,
    #[strum(serialize = "tg", message = "Tajik - tg")]
    Tajik,
    #[strum(serialize = "sd", message = "Sindhi - sd")]
    Sindhi,
    #[strum(serialize = "gu", message = "Gujarati - gu")]
    Gujarati,
    #[strum(serialize = "am", message = "Amharic - am")]
    Amharic,
    #[strum(serialize = "yi", message = "Yiddish - yi")]
    Yiddish,
    #[strum(serialize = "lo", message = "Lao - lo")]
    Lao,
    #[strum(serialize = "uz", message = "Uzbek - uz")]
    Uzbek,
    #[strum(serialize = "fo", message = "Faroese - fo")]
    Faroese,
    #[strum(serialize = "ht", message = "HaitianCreole - ht")]
    HaitianCreole,
    #[strum(serialize = "ps", message = "Pashto - ps")]
    Pashto,
    #[strum(serialize = "tk", message = "Turkmen - tk")]
    Turkmen,
    #[strum(serialize = "nn", message = "Nynorsk - nn")]
    Nynorsk,
    #[strum(serialize = "mt", message = "Maltese - mt")]
    Maltese,
    #[strum(serialize = "sa", message = "Sanskrit - sa")]
    Sanskrit,
    #[strum(serialize = "lb", message = "Luxembourgish - lb")]
    Luxembourgish,
    #[strum(serialize = "my", message = "Myanmar - my")]
    Myanmar,
    #[strum(serialize = "bo", message = "Tibetan - bo")]
    Tibetan,
    #[strum(serialize = "tl", message = "Tagalog - tl")]
    Tagalog,
    #[strum(serialize = "mg", message = "Malagasy - mg")]
    Malagasy,
    #[strum(serialize = "as", message = "Assamese - as")]
    Assamese,
    #[strum(serialize = "tt", message = "Tatar - tt")]
    Tatar,
    #[strum(serialize = "haw", message = "Hawaiian - haw")]
    Hawaiian,
    #[strum(serialize = "ln", message = "Lingala - ln")]
    Lingala,
    #[strum(serialize = "ha", message = "Hausa - ha")]
    Hausa,
    #[strum(serialize = "ba", message = "Bashkir - ba")]
    Bashkir,
    #[strum(serialize = "jw", message = "Javanese - jw")]
    Javanese,
    #[strum(serialize = "su", message = "Sundanese - su")]
    Sundanese,
}
