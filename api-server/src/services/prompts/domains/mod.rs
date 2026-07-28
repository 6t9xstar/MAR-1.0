pub mod law;
pub mod education;
pub mod business;
pub mod agriculture;
pub mod healthcare;
pub mod islamic;

use crate::knowledge::skills::skill_trait::Domain;
use std::collections::HashMap;

pub struct DomainPromptMap {
    prompts: HashMap<String, String>,
}

impl Default for DomainPromptMap {
    fn default() -> Self {
        Self::new()
    }
}

impl DomainPromptMap {
    pub fn new() -> Self {
        let mut prompts = HashMap::new();
        prompts.insert("Law".into(), law::LAW_PROMPT.to_string());
        prompts.insert("Education".into(), education::EDUCATION_PROMPT.to_string());
        prompts.insert("Business".into(), business::BUSINESS_PROMPT.to_string());
        prompts.insert("Agriculture".into(), agriculture::AGRICULTURE_PROMPT.to_string());
        prompts.insert("Healthcare".into(), healthcare::HEALTHCARE_PROMPT.to_string());
        prompts.insert("Islamic".into(), islamic::ISLAMIC_PROMPT.to_string());
        Self { prompts }
    }

    pub fn for_domain(&self, domain: &Domain) -> &str {
        self.prompts.get(&format!("{:?}", domain))
            .map(|s| s.as_str())
            .unwrap_or("")
    }
}
