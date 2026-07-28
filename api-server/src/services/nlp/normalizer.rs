use unicode_normalization::UnicodeNormalization;

pub struct UrduNormalizer;

impl Default for UrduNormalizer {
    fn default() -> Self {
        Self
    }
}

impl UrduNormalizer {
    pub fn new() -> Self {
        Self
    }

    pub fn normalize(&self, text: &str) -> String {
        let text = text.nfkc().collect::<String>();
        let text = self.normalize_alefs(&text);
        let text = self.normalize_ye(&text);
        let text = self.normalize_heh(&text);
        let text = self.normalize_hamza(&text);
        let text = self.normalize_kashida(&text);
        self.normalize_diacritics(&text)
    }

    fn normalize_alefs(&self, text: &str) -> String {
        text.chars().map(|c| match c {
            '\u{0622}' | '\u{0623}' | '\u{0625}' => '\u{0627}',
            '\u{0671}' | '\u{0672}' | '\u{0673}' => '\u{0627}',
            _ => c,
        }).collect()
    }

    fn normalize_ye(&self, text: &str) -> String {
        text.chars().map(|c| match c {
            '\u{064A}' | '\u{06CC}' | '\u{0649}' | '\u{06D2}' => '\u{06CC}',
            '\u{0626}' => '\u{0626}',
            _ => c,
        }).collect()
    }

    fn normalize_heh(&self, text: &str) -> String {
        text.chars().map(|c| match c {
            '\u{0629}' | '\u{06C3}' | '\u{06D0}' => '\u{06C1}',
            '\u{0647}' => '\u{06C1}',
            _ => c,
        }).collect()
    }

    fn normalize_hamza(&self, text: &str) -> String {
        text.chars().map(|c| match c {
            '\u{0624}' => '\u{0648}',
            '\u{0626}' => '\u{06CC}',
            '\u{0621}' => '\u{0621}',
            _ => c,
        }).collect()
    }

    fn normalize_kashida(&self, text: &str) -> String {
        text.replace('\u{0640}', "")
    }

    fn normalize_diacritics(&self, text: &str) -> String {
        text.chars().filter(|c| !matches!(c,
            '\u{064B}'..='\u{0652}' |
            '\u{0670}' |
            '\u{06D6}'..='\u{06ED}' |
            '\u{08D4}'..='\u{08FF}'
        )).collect()
    }
}
