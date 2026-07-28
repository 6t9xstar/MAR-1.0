use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Domain {
    Law,
    Education,
    Business,
    Agriculture,
    Healthcare,
    Islamic,
    Government,
    Geography,
    Culture,
    Coding,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    AnyKeyword(Vec<String>),
    AllKeywords(Vec<String>),
    Language(String),
    Intent(String),
    Always,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub source: String,
    pub url: Option<String>,
    pub text: String,
    pub authority: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeChunk {
    pub id: String,
    pub content: String,
    pub citations: Vec<Citation>,
    pub tags: Vec<String>,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub domain: Domain,
    pub version: String,
    pub activation_condition: Condition,
    pub confidence: f64,
    pub requires_disclaimer: bool,
    pub source_citations: Vec<Citation>,
    pub content_chunks: Vec<KnowledgeChunk>,
}

impl Skill {
    pub fn should_activate(&self, text: &str, language: &str) -> bool {
        let lower = text.to_lowercase();
        match &self.activation_condition {
            Condition::AnyKeyword(kws) => kws.iter().any(|kw| lower.contains(&kw.to_lowercase())),
            Condition::AllKeywords(kws) => kws.iter().all(|kw| lower.contains(&kw.to_lowercase())),
            Condition::Language(lang) => language == lang,
            Condition::Intent(_) => true,
            Condition::Always => true,
        }
    }

    pub fn relevant_chunks(&self, query: &str, max_chunks: usize) -> Vec<&KnowledgeChunk> {
        let lower = query.to_lowercase();
        let mut scored: Vec<(usize, &KnowledgeChunk)> = self.content_chunks.iter()
            .map(|chunk| {
                let score = chunk.tags.iter()
                    .filter(|t| lower.contains(&t.to_lowercase()))
                    .count();
                (score, chunk)
            })
            .collect();
        scored.sort_by_key(|b| std::cmp::Reverse(b.0));
        scored.into_iter()
            .take(max_chunks)
            .map(|(_, chunk)| chunk)
            .collect()
    }
}
