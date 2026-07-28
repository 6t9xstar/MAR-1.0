pub mod detector;
pub mod normalizer;
pub mod roman_urdu;

use crate::config::NlpConfig;

pub struct NlpService {
    pub detector: detector::LanguageDetector,
    pub normalizer: normalizer::UrduNormalizer,
    pub roman_urdu: roman_urdu::RomanUrduEngine,
}

impl NlpService {
    pub fn new(config: &NlpConfig) -> Self {
        Self {
            detector: detector::LanguageDetector::new(),
            normalizer: normalizer::UrduNormalizer::new(),
            roman_urdu: roman_urdu::RomanUrduEngine::new(
                config.roman_urdu_dict_path.as_deref(),
                config.punjabi_dict_path.as_deref(),
            ),
        }
    }

    pub fn detect_language(&self, text: &str) -> detector::Language {
        detector::LanguageDetector::detect(text)
    }

    pub fn normalize(&self, text: &str) -> String {
        self.normalizer.normalize(text)
    }

    pub fn transliterate_roman_urdu(&self, text: &str) -> String {
        self.roman_urdu.transliterate(text)
    }

    pub fn detect_span(&self, text: &str) -> Vec<detector::LanguageSpan> {
        detector::LanguageDetector::detect_span(text)
    }
}
