use crate::knowledge::skills::skill_trait::{
    Citation, Condition, Domain, KnowledgeChunk, Skill,
};

pub fn skill() -> Skill {
    Skill {
        id: "geography".into(),
        name: "Geography".into(),
        description: "Mountains, rivers, provinces, cities, climate, deserts, coast, Karakoram, Indus".into(),
        domain: Domain::Geography,
        version: "1.0.0".into(),
        activation_condition: Condition::AnyKeyword(vec![
            "geography".to_string(), "city".to_string(), "province".to_string(), "mountain".to_string(), "river".to_string(), "desert".to_string(), "sea".to_string(), "coast".to_string(), "karakoram".to_string(), "himalaya".to_string(), "hindukush".to_string(), "indus".to_string(), "punjab".to_string(), "sindh".to_string(), "kpk".to_string(), "balochistan".to_string(), "gilgit".to_string(), "kashmir".to_string(), "climate".to_string(), "weather".to_string(), "???????".to_string(), "???".to_string(), "????".to_string(), "????".to_string(), "????".to_string(),
        ]),
        confidence: 0.85,
        requires_disclaimer: false,
        source_citations: vec![
            Citation {
                source: "Survey of Pakistan".into(),
                url: Some("https://www.surveyofpakistan.gov.pk".into()),
                text: "Geographic data and maps".into(),
                authority: Some("Survey of Pakistan".into()),
            },
        ],
        content_chunks: vec![
            KnowledgeChunk {
                id: "physiography".into(),
                content: "Pakistan's geography: Northern mountains (Karakoram, Himalayas, Hindukush) - K2 (8,611m) world's 2nd highest. Indus River (3,180 km) - lifeline. Five rivers of Punjab: Indus, Jhelum, Chenab, Ravi, Sutlej. Deserts: Thar (Sindh), Cholistan (Punjab), Thal. Coastline: 1,046 km along Arabian Sea. Natural ports: Karachi, Gwadar, Port Qasim.".into(),
                citations: vec![],
                tags: vec!["mountains".into(), "rivers".into(), "indus".into(), "k2".into(), "deserts".into()],
                language: "en".into(),
            },
            KnowledgeChunk {
                id: "provinces".into(),
                content: "Pakistan's provinces: Punjab (largest population, agricultural heartland, capital Lahore), Sindh (capital Karachi - largest city), Khyber Pakhtunkhwa (KPK, mountainous, capital Peshawar), Balochistan (largest area, sparse population, capital Quetta). Territories: Islamabad Capital Territory, Gilgit-Baltistan, Azad Jammu and Kashmir.".into(),
                citations: vec![],
                tags: vec!["provinces".into(), "punjab".into(), "sindh".into(), "kpk".into(), "balochistan".into()],
                language: "en".into(),
            },
        ],
    }
}