use crate::knowledge::skills::skill_trait::{
    Citation, Condition, Domain, KnowledgeChunk, Skill,
};

pub fn skill() -> Skill {
    Skill {
        id: "business-economy".into(),
        name: "Business & Economy".into(),
        description: "Company registration, SECP, FBR, tax, NTN, sales tax, import/export, SME, corporate law, stock exchange".into(),
        domain: Domain::Business,
        version: "1.0.0".into(),
        activation_condition: Condition::AnyKeyword(vec![
            "business".to_string(), "company".to_string(), "registration".to_string(), "secp".to_string(), "fbr".to_string(), "tax".to_string(), "ntn".to_string(), "sales tax".to_string(), "income tax".to_string(), "import".to_string(), "export".to_string(), "sme".to_string(), "corporate".to_string(), "stock exchange".to_string(), "psx".to_string(), "trade".to_string(), "commerce".to_string(), "???????".to_string(), "?????".to_string(), "????".to_string(), "????????".to_string(),
        ]),
        confidence: 0.85,
        requires_disclaimer: true,
        source_citations: vec![
            Citation {
                source: "Securities and Exchange Commission of Pakistan (SECP)".into(),
                url: Some("https://www.secp.gov.pk".into()),
                text: "Company registration and regulation".into(),
                authority: Some("SECP".into()),
            },
            Citation {
                source: "Federal Board of Revenue (FBR)".into(),
                url: Some("https://www.fbr.gov.pk".into()),
                text: "Tax laws and procedures".into(),
                authority: Some("FBR".into()),
            },
        ],
        content_chunks: vec![
            KnowledgeChunk {
                id: "company-reg".into(),
                content: "Company registration in Pakistan is done through SECP's e-Services portal. Types: Single Member Company (SMC), Private Limited (Pvt Ltd), Public Limited, One Person Company (OPC). Requires: name reservation, Form 1 (declaration), memorandum and articles of association, NTN, bank account. Process takes 2-3 weeks online.".into(),
                citations: vec![],
                tags: vec!["registration".into(), "secp".into(), "company".into(), "incorporation".into()],
                language: "en".into(),
            },
            KnowledgeChunk {
                id: "tax-system".into(),
                content: "Pakistan tax system: Income Tax (individuals: 0-35% slabs, companies: 29%), Sales Tax (standard 18%), Corporate Tax. NTN (National Tax Number) is mandatory for all taxpayers. STRN (Sales Tax Registration Number) for businesses. FBR manages tax collection through Iris portal. Annual filing deadline: September 30 for individuals.".into(),
                citations: vec![],
                tags: vec!["tax".into(), "fbr".into(), "ntn".into(), "sales tax".into(), "income tax".into()],
                language: "en".into(),
            },
        ],
    }
}