use crate::knowledge::skills::skill_trait::{Domain, Skill};
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct SkillRegistry {
    skills: DashMap<String, Arc<Skill>>,
}

impl Default for SkillRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillRegistry {
    pub fn new() -> Self {
        Self {
            skills: DashMap::new(),
        }
    }

    pub fn register(&self, skill: Skill) {
        self.skills.insert(skill.id.clone(), Arc::new(skill));
    }

    pub fn get(&self, id: &str) -> Option<Arc<Skill>> {
        self.skills.get(id).map(|s| s.clone())
    }

    pub fn all(&self) -> Vec<Arc<Skill>> {
        self.skills.iter().map(|s| s.clone()).collect()
    }

    pub fn activate(&self, text: &str, language: &str) -> Vec<Arc<Skill>> {
        self.skills.iter()
            .filter(|s| s.should_activate(text, language))
            .map(|s| s.clone())
            .collect()
    }

    pub fn for_domain(&self, domain: &Domain) -> Vec<Arc<Skill>> {
        self.skills.iter()
            .filter(|s| s.domain == *domain)
            .map(|s| s.clone())
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.skills.is_empty()
    }

    pub fn len(&self) -> usize {
        self.skills.len()
    }
}
