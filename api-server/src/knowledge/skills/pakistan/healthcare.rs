use crate::knowledge::skills::skill_trait::{
    Citation, Condition, Domain, KnowledgeChunk, Skill,
};

pub fn skill() -> Skill {
    Skill {
        id: "healthcare".into(),
        name: "Healthcare".into(),
        description: "Hospitals, diseases, medicines, vaccination, health insurance, Sehat Card, COVID-19, child health".into(),
        domain: Domain::Healthcare,
        version: "1.0.0".into(),
        activation_condition: Condition::AnyKeyword(vec![
            "health".to_string(), "hospital".to_string(), "doctor".to_string(), "disease".to_string(), "medicine".to_string(), "vaccination".to_string(), "sehat card".to_string(), "covid".to_string(), "dengue".to_string(), "malaria".to_string(), "tuberculosis".to_string(), "diabetes".to_string(), "blood pressure".to_string(), "healthcare".to_string(), "clinic".to_string(), "???".to_string(), "??????".to_string(), "?????".to_string(), "????".to_string(),
        ]),
        confidence: 0.85,
        requires_disclaimer: true,
        source_citations: vec![
            Citation {
                source: "Ministry of National Health Services Pakistan".into(),
                url: Some("https://www.nhsrc.gov.pk".into()),
                text: "Healthcare policies and regulations".into(),
                authority: Some("NHSRC".into()),
            },
        ],
        content_chunks: vec![
            KnowledgeChunk {
                id: "health-system".into(),
                content: "Pakistan's healthcare system: Federal Ministry of National Health Services, provincial health departments. Major hospitals: Mayo Hospital Lahore, Jinnah Hospital Karachi, PIMS Islamabad. Sehat Sahulat Program (health insurance for low-income families). Lady Health Worker program covers ~60% of rural areas.".into(),
                citations: vec![],
                tags: vec!["system".into(), "hospitals".into(), "sehat card".into(), "health insurance".into()],
                language: "en".into(),
            },
            KnowledgeChunk {
                id: "diseases".into(),
                content: "Major health challenges in Pakistan: Infectious diseases (Dengue, Malaria, Hepatitis B/C, TB, COVID-19), Non-communicable (Diabetes - 33% adults, Hypertension, Cardiovascular). Child health: 40% children stunted due to malnutrition, EPI program covers 10 vaccines. Polio remains endemic - Pakistan and Afghanistan only countries.".into(),
                citations: vec![],
                tags: vec!["diseases".into(), "diabetes".into(), "dengue".into(), "polio".into(), "malaria".into()],
                language: "en".into(),
            },
        ],
    }
}