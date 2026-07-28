use crate::knowledge::skills::skill_trait::{
    Citation, Condition, Domain, KnowledgeChunk, Skill,
};

pub fn skill() -> Skill {
    Skill {
        id: "islamic".into(),
        name: "Islamic".into(),
        description: "Quran, Hadith, Salah, Zakat, Hajj, Umrah, Islamic finance, Fiqh, Shariah".into(),
        domain: Domain::Islamic,
        version: "1.0.0".into(),
        activation_condition: Condition::AnyKeyword(vec![
            "islam".to_string(), "quran".to_string(), "surah".to_string(), "ayah".to_string(), "hadith".to_string(), "bukhari".to_string(), "muslim".to_string(), "salah".to_string(), "namaz".to_string(), "zakat".to_string(), "hajj".to_string(), "umrah".to_string(), "roza".to_string(), "fasting".to_string(), "islamic finance".to_string(), "shariah".to_string(), "fiqh".to_string(), "fatwa".to_string(), "?????".to_string(), "??".to_string(), "?????".to_string(), "????".to_string(), "????".to_string(),
        ]),
        confidence: 0.85,
        requires_disclaimer: true,
        source_citations: vec![
            Citation {
                source: "Sahih al-Bukhari".into(),
                url: Some("https://sunnah.com/bukhari".into()),
                text: "Most authentic Hadith collection".into(),
                authority: Some("Imam Bukhari".into()),
            },
            Citation {
                source: "Quran.com".into(),
                url: Some("https://quran.com".into()),
                text: "Quran text and translations".into(),
                authority: Some("".into()),
            },
        ],
        content_chunks: vec![
            KnowledgeChunk {
                id: "five-pillars".into(),
                content: "The Five Pillars of Islam: 1) Shahada (declaration of faith - 'La ilaha illallah, Muhammadur Rasulullah'), 2) Salah (five daily prayers - Fajr, Dhuhr, Asr, Maghrib, Isha), 3) Zakat (obligatory charity - 2.5% of savings), 4) Sawm (fasting during Ramadan), 5) Hajj (pilgrimage to Mecca, once in lifetime if able).".into(),
                citations: vec![],
                tags: vec!["pillars".into(), "salah".into(), "zakat".into(), "hajj".into(), "roza".into()],
                language: "en".into(),
            },
            KnowledgeChunk {
                id: "islamic-finance".into(),
                content: "Islamic finance principles: Riba (interest) prohibited under Shariah. Halal alternatives: Mudarabah (profit-sharing), Musharakah (joint venture), Ijarah (leasing), Murabahah (cost-plus financing). Pakistan's Federal Shariat Court declared interest unconstitutional. State Bank of Pakistan promotes Islamic banking - currently ~20% of banking sector.".into(),
                citations: vec![],
                tags: vec!["finance".into(), "shariah".into(), "riba".into(), "islamic banking".into()],
                language: "en".into(),
            },
        ],
    }
}