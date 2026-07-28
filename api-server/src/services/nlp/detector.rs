use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    English,
    Urdu,
    RomanUrdu,
    Punjabi,
    Mixed,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct LanguageSpan {
    pub language: Language,
    pub start: usize,
    pub end: usize,
    pub text: String,
}

pub struct LanguageDetector;

impl Default for LanguageDetector {
    fn default() -> Self {
        Self
    }
}

impl LanguageDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(text: &str) -> Language {
        if text.is_empty() {
            return Language::Unknown;
        }

        let chars: Vec<char> = text.chars().collect();
        let mut urdu_count = 0;
        let mut en_count = 0;
        let mut devanagari_count = 0;
        let total = chars.len() as f64;

        for &c in &chars {
            match c {
                '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{08A0}'..='\u{08FF}' => {
                    urdu_count += 1;
                }
                '\u{0041}'..='\u{005A}' | '\u{0061}'..='\u{007A}' => {
                    en_count += 1;
                }
                '\u{0900}'..='\u{097F}' | '\u{0A00}'..='\u{0A7F}' => {
                    devanagari_count += 1;
                }
                _ => {}
            }
        }

        let urdu_ratio = urdu_count as f64 / total;
        let en_ratio = en_count as f64 / total;
        let dev_ratio = devanagari_count as f64 / total;

        if dev_ratio > 0.3 {
            return Language::Punjabi;
        }

        if urdu_ratio > 0.4 {
            if en_ratio > 0.2 {
                return Language::Mixed;
            }
            return Language::Urdu;
        }

        if en_ratio > 0.5 && Self::has_urdu_lexicon(text) {
            return Language::RomanUrdu;
        }

        if urdu_ratio > 0.1 && en_ratio > 0.1 {
            return Language::Mixed;
        }

        if en_ratio > 0.5 {
            return Language::English;
        }

        Language::Unknown
    }

    pub fn detect_span(text: &str) -> Vec<LanguageSpan> {
        let mut spans = Vec::new();
        let graphemes: Vec<&str> = text.graphemes(true).collect();
        if graphemes.is_empty() {
            return spans;
        }

        let mut start = 0usize;
        let mut current_lang = Self::detect_single(graphemes[0]);

        for (i, g) in graphemes.iter().enumerate().skip(1) {
            let lang = Self::detect_single(g);
            if lang != current_lang {
                let span_text: String = graphemes[start..=i-1].concat();
                spans.push(LanguageSpan {
                    language: current_lang,
                    start,
                    end: i,
                    text: span_text,
                });
                start = i;
                current_lang = lang;
            }
        }

        let span_text: String = graphemes[start..].concat();
        spans.push(LanguageSpan {
            language: current_lang,
            start,
            end: graphemes.len(),
            text: span_text,
        });

        spans
    }

    fn detect_single(text: &str) -> Language {
        for c in text.chars() {
            match c {
                '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{08A0}'..='\u{08FF}' => {
                    return Language::Urdu;
                }
                '\u{0900}'..='\u{097F}' | '\u{0A00}'..='\u{0A7F}' => {
                    return Language::Punjabi;
                }
                '\u{0041}'..='\u{005A}' | '\u{0061}'..='\u{007A}' => {
                    return Language::English;
                }
                _ => {}
            }
        }
        Language::Unknown
    }

    fn has_urdu_lexicon(text: &str) -> bool {
        let urdu_words = [
            "hai", "hain", "ka", "ki", "ke", "ko", "se", "mein", "aur", "yeh",
            "woh", "mera", "tera", "apna", "hum", "tum", "aap", "kya", "kyun",
            "kahan", "kaise", "acha", "theek", "nahi", "haan", "ji", "sahi",
            "ghalt", "bat", "kam", "zara", "thoda", "bohat", "chahiye",
            "kar", "ho", "ja", "aa", "de", "le", "rakh",
            "saktay", "sakta", "sakti", "chahiye", "hoga", "hogay",
        ];
        let lower = text.to_lowercase();
        urdu_words.iter().any(|w| lower.contains(w))
    }

    pub fn confidence(text: &str) -> f64 {
        let lang = Self::detect(text);
        match lang {
            Language::English | Language::Urdu => 0.9,
            Language::RomanUrdu => 0.7,
            Language::Mixed => 0.5,
            Language::Punjabi => 0.6,
            Language::Unknown => 0.0,
        }
    }
}
