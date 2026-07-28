use crate::knowledge::skills::skill_trait::{Citation, Domain, KnowledgeChunk};

pub struct CitationService;

impl Default for CitationService {
    fn default() -> Self {
        Self
    }
}

impl CitationService {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_claims(_text: &str) -> Vec<String> {
        vec![]
    }

    pub fn format_citations(citations: &[Citation], format: CitationFormat) -> String {
        if citations.is_empty() {
            return String::new();
        }
        match format {
            CitationFormat::Inline => {
                let sources: Vec<&str> = citations.iter().map(|c| c.source.as_str()).collect();
                format!("[Sources: {}]", sources.join(", "))
            }
            CitationFormat::Footnotes => {
                citations.iter().enumerate()
                    .map(|(i, c)| format!("[{}] {} - {}", i + 1, c.source, c.text))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            CitationFormat::Endnotes => {
                format!("\n\n**References:**\n{}", citations.iter().enumerate()
                    .map(|(i, c)| {
                        let url = c.url.as_ref().map(|u| format!(" ({})", u)).unwrap_or_default();
                        format!("{}. {}{}", i + 1, c.source, url)
                    })
                    .collect::<Vec<_>>()
                    .join("\n"))
            }
        }
    }

    pub fn inject_disclaimer(domain: &Domain) -> Option<&'static str> {
        match domain {
            Domain::Healthcare => Some("DISCLAIMER: I am an AI assistant, not a doctor. This information is for educational purposes only and should not replace professional medical advice."),
            Domain::Law => Some("DISCLAIMER: I am an AI assistant, not a lawyer. This information is for general informational purposes only and does not constitute legal advice."),
            Domain::Islamic => Some("DISCLAIMER: I am an AI assistant, not a religious scholar. Please verify religious rulings with qualified Islamic scholars."),
            Domain::Business => Some("DISCLAIMER: I am an AI assistant, not a financial advisor. This information is for general guidance and should not replace professional financial or legal advice."),
            _ => None,
        }
    }

    pub fn confidence_score(_citations: &[Citation], chunks: &[KnowledgeChunk]) -> f64 {
        if chunks.is_empty() {
            return 0.0;
        }
        let base = chunks.iter()
            .map(|c| c.citations.len() as f64)
            .sum::<f64>() / chunks.len() as f64;
        (base * 0.2).clamp(0.1, 0.95)
    }
}

pub enum CitationFormat {
    Inline,
    Footnotes,
    Endnotes,
}
