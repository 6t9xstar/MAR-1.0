use crate::knowledge::skills::skill_trait::{
    Citation, Condition, Domain, KnowledgeChunk, Skill,
};

pub fn skill() -> Skill {
    Skill {
        id: "government".into(),
        name: "Government".into(),
        description: "Government structure, ministries, constitution, parliament, Prime Minister, President, provinces, local government".into(),
        domain: Domain::Government,
        version: "1.0.0".into(),
        activation_condition: Condition::AnyKeyword(vec![
            "government".to_string(), "ministry".to_string(), "minister".to_string(), "prime minister".to_string(), "president".to_string(), "parliament".to_string(), "senate".to_string(), "national assembly".to_string(), "provincial assembly".to_string(), "chief minister".to_string(), "governor".to_string(), "election".to_string(), "ecp".to_string(), "constitution".to_string(), "local government".to_string(), "bureaucracy".to_string(), "?????".to_string(), "????".to_string(), "????????".to_string(), "????????".to_string(),
        ]),
        confidence: 0.85,
        requires_disclaimer: false,
        source_citations: vec![
            Citation {
                source: "Constitution of Pakistan 1973".into(),
                url: Some("https://na.gov.pk/en/constitution.php".into()),
                text: "Constitutional framework".into(),
                authority: Some("National Assembly of Pakistan".into()),
            },
            Citation {
                source: "Election Commission of Pakistan".into(),
                url: Some("https://www.ecp.gov.pk".into()),
                text: "Election procedures and results".into(),
                authority: Some("ECP".into()),
            },
        ],
        content_chunks: vec![
            KnowledgeChunk {
                id: "govt-structure".into(),
                content: "Pakistan's government structure: Federal parliamentary democratic republic. President (head of state), Prime Minister (head of government). Parliament: National Assembly (lower house, 336 seats including 60 women and 10 non-Muslim), Senate (upper house, 96 seats). Four provinces: Punjab, Sindh, KPK, Balochistan. Each has Provincial Assembly and Chief Minister.".into(),
                citations: vec![],
                tags: vec!["structure".into(), "parliament".into(), "pm".into(), "president".into(), "provinces".into()],
                language: "en".into(),
            },
            KnowledgeChunk {
                id: "electoral-system".into(),
                content: "Pakistan's electoral system: Election Commission of Pakistan (ECP) conducts elections. National Assembly members elected through first-past-the-post in 266 constituencies. Senate elected by provincial assemblies. Local government: divisions, districts, tehsils, union councils. Voting age: 18. Women and non-Muslims have reserved seats.".into(),
                citations: vec![],
                tags: vec!["elections".into(), "ecp".into(), "voting".into(), "constituencies".into()],
                language: "en".into(),
            },
        ],
    }
}