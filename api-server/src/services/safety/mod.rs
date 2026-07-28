mod medical;
mod legal;
mod religious;
mod financial;

use crate::knowledge::skills::skill_trait::Domain;

pub struct SafetyService;

impl Default for SafetyService {
    fn default() -> Self {
        Self
    }
}

impl SafetyService {
    pub fn new() -> Self {
        Self
    }

    pub fn disclaimer_for(&self, domain: &Domain) -> Option<&'static str> {
        match domain {
            Domain::Healthcare => Some(medical::MEDICAL_DISCLAIMER),
            Domain::Law => Some(legal::LEGAL_DISCLAIMER),
            Domain::Islamic => Some(religious::RELIGIOUS_DISCLAIMER),
            Domain::Business => Some(financial::FINANCIAL_DISCLAIMER),
            _ => None,
        }
    }

    pub fn disclaimer_prefix(&self, text: &str, domain: &Domain) -> String {
        match self.disclaimer_for(domain) {
            Some(d) => format!("{}\n\n{}", d, text),
            None => text.to_string(),
        }
    }
}
