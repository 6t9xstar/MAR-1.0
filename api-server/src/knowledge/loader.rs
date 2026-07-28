use crate::knowledge::skills::registry::SkillRegistry;
use crate::knowledge::skills::skill_trait::Skill;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

pub struct KnowledgeLoader {
    data_dir: String,
}

impl KnowledgeLoader {
    pub fn new(data_dir: &str) -> Self {
        Self {
            data_dir: data_dir.to_string(),
        }
    }

    pub async fn ingest_all(&self, registry: &SkillRegistry) {
        let path = Path::new(&self.data_dir);
        if !path.exists() {
            warn!(dir = %self.data_dir, "Knowledge data directory not found");
            return;
        }

        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(e) => {
                warn!(error = %e, "Failed to read knowledge directory");
                return;
            }
        };

        for entry in entries.flatten() {
            let fpath = entry.path();
            if fpath.extension().map(|e| e == "yaml" || e == "yml").unwrap_or(false) {
                match self.ingest_file(&fpath) {
                    Ok(skill) => {
                        registry.register(skill);
                        info!(file = %fpath.display(), "Ingested knowledge file");
                    }
                    Err(e) => {
                        warn!(file = %fpath.display(), error = %e, "Failed to ingest knowledge file");
                    }
                }
            }
        }
    }

    fn ingest_file(&self, path: &Path) -> eyre::Result<Skill> {
        let content = fs::read_to_string(path)?;
        let skill: Skill = serde_yaml::from_str(&content)?;
        Ok(skill)
    }
}
