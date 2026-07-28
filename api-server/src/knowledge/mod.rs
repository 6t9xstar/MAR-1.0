pub mod skills;
pub mod loader;
pub mod citation;

use crate::knowledge::skills::registry::SkillRegistry;
use crate::knowledge::skills::skill_trait::{Domain, KnowledgeChunk};
use std::sync::Arc;

pub struct KnowledgeService {
    pub registry: SkillRegistry,
    loader: loader::KnowledgeLoader,
}

impl KnowledgeService {
    pub fn new(data_dir: &str) -> Self {
        let registry = SkillRegistry::new();
        let loader = loader::KnowledgeLoader::new(data_dir);
        Self { registry, loader }
    }

    pub async fn ingest_all(&self) {
        self.loader.ingest_all(&self.registry).await;
    }

    pub fn activate_skills(&self, text: &str, language: &str) -> Vec<Arc<skills::skill_trait::Skill>> {
        self.registry.activate(text, language)
    }

    pub fn domain_for_text(&self, text: &str, language: &str) -> Domain {
        let skills = self.activate_skills(text, language);
        if skills.is_empty() {
            return Domain::General;
        }
        skills[0].domain.clone()
    }

    pub fn relevant_knowledge(&self, text: &str, language: &str, max_chunks: usize) -> Vec<KnowledgeChunk> {
        let skills = self.activate_skills(text, language);
        let mut chunks = Vec::new();
        for skill in &skills {
            for chunk in skill.relevant_chunks(text, max_chunks) {
                if chunks.len() < max_chunks {
                    chunks.push(chunk.clone());
                }
            }
        }
        chunks
    }

    pub fn requires_disclaimer(&self, text: &str, language: &str) -> Option<Domain> {
        let skills = self.activate_skills(text, language);
        for skill in &skills {
            if skill.requires_disclaimer {
                return Some(skill.domain.clone());
            }
        }
        None
    }
}
