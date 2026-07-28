use crate::knowledge::skills::skill_trait::{
    Citation, Condition, Domain, KnowledgeChunk, Skill,
};

pub fn skill() -> Skill {
    Skill {
        id: "culture".into(),
        name: "Culture".into(),
        description: "Languages, festivals, food, music, traditions, art, literature, Sufism".into(),
        domain: Domain::Culture,
        version: "1.0.0".into(),
        activation_condition: Condition::AnyKeyword(vec![
            "culture".to_string(), "language".to_string(), "festival".to_string(), "food".to_string(), "tradition".to_string(), "music".to_string(), "art".to_string(), "literature".to_string(), "sufi".to_string(), "qawwali".to_string(), "urdu".to_string(), "punjabi".to_string(), "pashto".to_string(), "sindhi".to_string(), "balochi".to_string(), "eid".to_string(), "shab-e-barat".to_string(), "basant".to_string(), "melad".to_string(), "?????".to_string(), "????".to_string(), "?????".to_string(), "?????".to_string(), "?????".to_string(),
        ]),
        confidence: 0.85,
        requires_disclaimer: false,
        source_citations: vec![
            Citation {
                source: "Pakistan National Council of the Arts (PNCA)".into(),
                url: Some("https://www.pnca.org.pk".into()),
                text: "Cultural heritage and events".into(),
                authority: Some("PNCA".into()),
            },
        ],
        content_chunks: vec![
            KnowledgeChunk {
                id: "languages".into(),
                content: "Pakistan's linguistic diversity: National language: Urdu (lingua franca, ~8% native). Regional languages: Punjabi (most spoken, ~40%), Pashto (~15%), Sindhi (~14%), Saraiki (~10%), Balochi (~4%), Hindko (~3%), Brahvi. English is official language of government and education. Pakistan has 74+ spoken languages.".into(),
                citations: vec![],
                tags: vec!["languages".into(), "urdu".into(), "punjabi".into(), "pashto".into(), "sindhi".into()],
                language: "en".into(),
            },
            KnowledgeChunk {
                id: "festivals".into(),
                content: "Major festivals in Pakistan: Eid-ul-Fitr (end of Ramadan), Eid-ul-Adha (festival of sacrifice), Independence Day (August 14), Defence Day (September 6), Pakistan Day (March 23), Iqbal Day (November 9), Quaid-e-Azam Day (December 25). Cultural festivals: Urs of saints (Data Ganj Bakhsh, Lal Shahbaz Qalandar), Basant (spring kite festival - restricted), Shab-e-Barat, Nowruz.".into(),
                citations: vec![],
                tags: vec!["festivals".into(), "eid".into(), "urs".into(), "independence".into(), "cultural".into()],
                language: "en".into(),
            },
        ],
    }
}