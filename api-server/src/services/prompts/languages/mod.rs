pub mod en;
pub mod ur;
pub mod roman_urdu;

use crate::services::nlp::detector::Language;
use std::collections::HashMap;

pub struct LanguagePromptMap {
    prompts: HashMap<String, String>,
}

impl Default for LanguagePromptMap {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguagePromptMap {
    pub fn new() -> Self {
        let mut prompts = HashMap::new();
        prompts.insert("English".into(), en::EN_PROMPT.to_string());
        prompts.insert("Urdu".into(), ur::UR_PROMPT.to_string());
        prompts.insert("RomanUrdu".into(), roman_urdu::ROMAN_URDU_PROMPT.to_string());
        Self { prompts }
    }

    pub fn for_language(&self, language: &Language) -> String {
        let key = format!("{:?}", language);
        self.prompts.get(&key)
            .cloned()
            .unwrap_or_else(|| self.prompts.get("English").cloned().unwrap_or_default())
    }
}
