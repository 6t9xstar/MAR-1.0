use crate::knowledge::skills::skill_trait::{
    Citation, Condition, Domain, KnowledgeChunk, Skill,
};

pub fn skill() -> Skill {
    Skill {
        id: "agriculture".into(),
        name: "Agriculture".into(),
        description: "Crops, farming, livestock, irrigation, Kissan Card, subsidies, Punjab agriculture, Sindh agriculture".into(),
        domain: Domain::Agriculture,
        version: "1.0.0".into(),
        activation_condition: Condition::AnyKeyword(vec![
            "agriculture".to_string(), "farming".to_string(), "crop".to_string(), "wheat".to_string(), "rice".to_string(), "cotton".to_string(), "sugarcane".to_string(), "maize".to_string(), "livestock".to_string(), "poultry".to_string(), "dairy".to_string(), "irrigation".to_string(), "kissan card".to_string(), "fertilizer".to_string(), "pesticide".to_string(), "?????".to_string(), "????? ????".to_string(), "???".to_string(),
        ]),
        confidence: 0.85,
        requires_disclaimer: false,
        source_citations: vec![
            Citation {
                source: "Pakistan Ministry of National Food Security and Research".into(),
                url: Some("https://www.mnfsr.gov.pk".into()),
                text: "Agricultural policies and data".into(),
                authority: Some("MNFSR".into()),
            },
        ],
        content_chunks: vec![
            KnowledgeChunk {
                id: "major-crops".into(),
                content: "Pakistan's major crops: Wheat (largest food crop, Punjab heartland), Rice (Basmati - world-renowned, mainly Punjab), Cotton (cash crop, backbone of textile industry - Punjab and Sindh), Sugarcane (Punjab and KPK). Minor crops: Maize, pulses, vegetables. Pakistan is among top 10 producers of wheat, rice, and cotton globally.".into(),
                citations: vec![],
                tags: vec!["crops".into(), "wheat".into(), "rice".into(), "cotton".into(), "sugarcane".into()],
                language: "en".into(),
            },
            KnowledgeChunk {
                id: "irrigation".into(),
                content: "Pakistan has the world's largest contiguous irrigation system: Indus Basin Irrigation System. 3 major dams (Tarbela, Mangla, Chashma), 19 barrages, 12 inter-river canals, 45 canal commands. Total canal length: ~60,000 km. Irrigates ~18 million hectares. Issues: water logging, salinity, climate change impact on Indus flows.".into(),
                citations: vec![],
                tags: vec!["irrigation".into(), "water".into(), "indus".into(), "dams".into()],
                language: "en".into(),
            },
        ],
    }
}