use crate::knowledge::skills::skill_trait::{
    Citation, Condition, Domain, KnowledgeChunk, Skill,
};

pub fn skill() -> Skill {
    Skill {
        id: "education".into(),
        name: "Education".into(),
        description: "Education system, boards, universities, HEC, admissions, scholarships, curriculum".into(),
        domain: Domain::Education,
        version: "1.0.0".into(),
        activation_condition: Condition::AnyKeyword(vec![
            "education".to_string(), "university".to_string(), "college".to_string(), "school".to_string(), "board".to_string(), "admission".to_string(), "scholarship".to_string(), "hec".to_string(), "fbise".to_string(), "bise".to_string(), "matric".to_string(), "intermediate".to_string(), "bachelor".to_string(), "master".to_string(), "phd".to_string(), "merit".to_string(), "entry test".to_string(), "nust".to_string(), "fast".to_string(), "lums".to_string(), "aku".to_string(), "pu".to_string(), "??????".to_string(), "?????????".to_string(), "?????".to_string(), "?????".to_string(),
        ]),
        confidence: 0.85,
        requires_disclaimer: false,
        source_citations: vec![
            Citation {
                source: "Higher Education Commission Pakistan".into(),
                url: Some("https://www.hec.gov.pk".into()),
                text: "HEC policies and university rankings".into(),
                authority: Some("HEC Pakistan".into()),
            },
            Citation {
                source: "Federal Board of Intermediate and Secondary Education".into(),
                url: Some("https://www.fbise.edu.pk".into()),
                text: "FBISE examination system".into(),
                authority: Some("FBISE".into()),
            },
        ],
        content_chunks: vec![
            KnowledgeChunk {
                id: "edu-system".into(),
                content: "Pakistan's education system: 5+4+2+4 structure (5 years primary, 4 middle, 2 secondary, 4 higher secondary). Boards of Intermediate and Secondary Education (BISE) conduct exams for grades 9-12. Higher Education Commission (HEC) oversees universities. Punjab, Sindh, KPK, Balochistan have their own education departments.".into(),
                citations: vec![],
                tags: vec!["system".into(), "structure".into(), "boards".into(), "hec".into()],
                language: "en".into(),
            },
            KnowledgeChunk {
                id: "universities".into(),
                content: "Top Pakistani universities: National University of Sciences and Technology (NUST), Lahore University of Management Sciences (LUMS), Pakistan Institute of Engineering and Applied Sciences (PIEAS), University of the Punjab, Aga Khan University, University of Karachi, University of Engineering and Technology (UET) Lahore.".into(),
                citations: vec![],
                tags: vec!["universities".into(), "nust".into(), "lums".into(), "rankings".into()],
                language: "en".into(),
            },
        ],
    }
}