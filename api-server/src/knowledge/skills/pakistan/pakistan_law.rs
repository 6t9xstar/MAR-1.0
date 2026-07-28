use crate::knowledge::skills::skill_trait::{
    Citation, Condition, Domain, KnowledgeChunk, Skill,
};

pub fn skill() -> Skill {
    Skill {
        id: "pakistan-law".into(),
        name: "Pakistan Law".into(),
        description: "Constitution, Penal Code, family law, property law, court procedures, FIA, NADRA".into(),
        domain: Domain::Law,
        version: "1.0.0".into(),
        activation_condition: Condition::AnyKeyword(vec![
            "law".to_string(), "legal".to_string(), "court".to_string(), "case".to_string(), "lawyer".to_string(), "judge".to_string(), "supreme court".to_string(), "high court".to_string(), "penal code".to_string(), "constitution".to_string(), "crpc".to_string(), "fia".to_string(), "nadra".to_string(), "family law".to_string(), "property".to_string(), "inheritance".to_string(), "will".to_string(), "divorce".to_string(), " custody".to_string(), "bail".to_string(), "fir".to_string(), "complaint".to_string(), "petition".to_string(), "appeal".to_string(), "tribunal".to_string(), "?????".to_string(), "?????".to_string(), "???".to_string(), "????".to_string(), "??".to_string(), "??? ??? ??".to_string(), "?????".to_string(),
        ]),
        confidence: 0.85,
        requires_disclaimer: true,
        source_citations: vec![
            Citation {
                source: "Constitution of Pakistan 1973".into(),
                url: Some("https://na.gov.pk/en/constitution.php".into()),
                text: "Supreme law of Pakistan".into(),
                authority: Some("National Assembly of Pakistan".into()),
            },
            Citation {
                source: "Pakistan Penal Code 1860 (Act XLV)".into(),
                url: Some("https://www.pakistani.org/pakistan/legislation/1860/actXLVof1860.html".into()),
                text: "Main criminal code of Pakistan".into(),
                authority: Some("Pakistan Legislature".into()),
            },
        ],
        content_chunks: vec![
            KnowledgeChunk {
                id: "const-basics".into(),
                content: "The Constitution of Pakistan 1973 is the supreme law. It establishes Pakistan as a federal parliamentary democratic republic with Islam as the state religion. Key features: Fundamental Rights (Articles 8-28), Principles of Policy (Articles 29-40), Federal (Articles 41-100) and Provincial governments.".into(),
                citations: vec![],
                tags: vec!["constitution".into(), "government".into(), "rights".into()],
                language: "en".into(),
            },
            KnowledgeChunk {
                id: "court-system".into(),
                content: "Pakistan's court system: Supreme Court (apex court, original/appellate/revisory jurisdiction), High Courts (one for each province + Islamabad), District & Sessions Courts (civil and criminal), Special Courts (Banking, Customs, Anti-Terrorism, Family, Juvenile). Federal Shariat Court examines laws for repugnancy to Islam.".into(),
                citations: vec![],
                tags: vec!["courts".into(), "judiciary".into(), "supreme court".into(), "high court".into()],
                language: "en".into(),
            },
            KnowledgeChunk {
                id: "family-law".into(),
                content: "Family laws in Pakistan: Muslim Family Laws Ordinance 1961 (marriage, divorce, maintenance, custody), Child Marriage Restraint Act, Guardian and Wards Act 1890, Dissolution of Muslim Marriages Act 1939. Family courts handle these matters. Nikah registration is mandatory under the Muslim Family Laws Ordinance.".into(),
                citations: vec![],
                tags: vec!["family".into(), "marriage".into(), "divorce".into(), "nikah".into(), "custody".into()],
                language: "en".into(),
            },
        ],
    }
}