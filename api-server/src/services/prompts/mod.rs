pub mod base;
pub mod domains;
pub mod languages;

use crate::knowledge::skills::skill_trait::Domain;
use crate::knowledge::KnowledgeService;
use crate::services::nlp::detector::Language;

pub struct PromptBuilder {
    base: base::BasePrompt,
    domain_prompts: domains::DomainPromptMap,
    language_prompts: languages::LanguagePromptMap,
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            base: base::BasePrompt::new(),
            domain_prompts: domains::DomainPromptMap::new(),
            language_prompts: languages::LanguagePromptMap::new(),
        }
    }

    pub fn build(
        &self,
        language: &Language,
        domain: &Domain,
        knowledge: Option<&KnowledgeService>,
        memory_context: Option<&str>,
        user_text: &str,
    ) -> String {
        let mut parts: Vec<String> = Vec::new();

        parts.push(self.base.system_prompt().to_string());

        let lang_prompt = self.language_prompts.for_language(language);
        parts.push(lang_prompt);

        let domain_prompt = self.domain_prompts.for_domain(domain);
        if !domain_prompt.is_empty() {
            parts.push(domain_prompt.to_string());
        }

        if let Some(mem) = memory_context
            && !mem.is_empty()
        {
            parts.push(format!("### User Context\n{}", mem));
        }

        if let Some(knowledge) = knowledge {
            let chunks = knowledge.relevant_knowledge(user_text, &format!("{:?}", language).to_lowercase(), 5);
            if !chunks.is_empty() {
                let knowledge_str: Vec<String> = chunks.iter()
                    .map(|c| c.content.clone())
                    .collect();
                parts.push(format!("### Relevant Knowledge\n{}", knowledge_str.join("\n\n")));
            }
        }

        parts.join("\n\n")
    }
}
