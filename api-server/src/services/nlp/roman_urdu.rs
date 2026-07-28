use std::collections::HashMap;

pub struct RomanUrduEngine {
    lookup: HashMap<String, String>,
    #[allow(dead_code)]
    punjabi_lookup: HashMap<String, String>,
}

impl RomanUrduEngine {
    pub fn new(_roman_dict_path: Option<&str>, _punjabi_dict_path: Option<&str>) -> Self {
        let mut lookup = HashMap::new();
        Self::seed_common_words(&mut lookup);
        Self {
            lookup,
            punjabi_lookup: HashMap::new(),
        }
    }

    pub fn transliterate(&self, text: &str) -> String {
        let words = text.split_whitespace();
        let mut result = String::new();

        for word in words {
            let clean = word.trim_matches(|c: char| c.is_ascii_punctuation());
            let transliterated = self.lookup.get(&clean.to_lowercase())
                .cloned()
                .unwrap_or_else(|| self.rule_based_transliterate(clean));
            result.push_str(&transliterated);
            result.push(' ');
        }

        result.trim().to_string()
    }

    fn rule_based_transliterate(&self, word: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = word.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];
            let next = chars.get(i + 1).copied().unwrap_or(' ');

            let urdu = match (c, next) {
                ('a', _) if i == 0 && next == 'i' => { i += 1; "عی".to_string() }
                ('a', _) if i == 0 && next == 'u' => { i += 1; "او".to_string() }
                ('a', _) if chars.len() > 2 && i == chars.len() - 1 => "ہ".to_string(),
                ('a', 'a') => { i += 1; "آ".to_string() }
                ('a', _) => "ا".to_string(),
                ('b', _) => "ب".to_string(),
                ('p', _) => "پ".to_string(),
                ('t', 't') | ('t', 'h') if next == 'h' || (next == 't' && chars.get(i+2) == Some(&'h')) => {
                    if next == 'h' { i += 1; } else { i += 2; }
                    "ٹھ".to_string()
                }
                ('t', 'h') => { i += 1; "تھ".to_string() }
                ('t', _) => "ت".to_string(),
                ('T', _) | ('ṭ', _) => "ٹ".to_string(),
                ('s', 'h') => { i += 1; "ش".to_string() }
                ('s', _) => "س".to_string(),
                ('S', _) => "ص".to_string(),
                ('j', _) => "ج".to_string(),
                ('c', 'h') => { i += 1; "چ".to_string() }
                ('h', _) => "ہ".to_string(),
                ('k', 'h') => { i += 1; "کھ".to_string() }
                ('k', _) => "ک".to_string(),
                ('q', _) => "ق".to_string(),
                ('d', 'd' | 'h') if next == 'h' || (next == 'd' && chars.get(i+2) == Some(&'h')) => {
                    if next == 'h' { i += 1; } else { i += 2; }
                    "ڈھ".to_string()
                }
                ('d', 'h') => { i += 1; "دھ".to_string() }
                ('d', _) => "د".to_string(),
                ('D', _) | ('ḍ', _) => "ڈ".to_string(),
                ('r', _) => "ر".to_string(),
                ('R', _) | ('ṛ', _) => "ڑ".to_string(),
                ('Z', _) => "ظ".to_string(),
                ('z', 'h') => { i += 1; "ظ".to_string() }
                ('z', _) => "ز".to_string(),
                ('g', 'h') => { i += 1; "گھ".to_string() }
                ('g', _) => "گ".to_string(),
                ('f', _) => "ف".to_string(),
                ('l', _) => "ل".to_string(),
                ('m', _) => "م".to_string(),
                ('n', 'g') => { i += 1; "نگ".to_string() }
                ('n', _) => "ن".to_string(),
                ('v', _) | ('w', _) => "و".to_string(),
                ('y', _) => "ی".to_string(),
                ('e', _) if i == 0 => "ای".to_string(),
                ('e', _) => "ے".to_string(),
                ('i', _) => "ی".to_string(),
                ('o', _) => "او".to_string(),
                ('u', _) => "ا".to_string(),
                ('N', _) => "ن".to_string(),
                _ => c.to_string(),
            };
            result.push_str(&urdu);
            i += 1;
        }

        result
    }

    fn seed_common_words(lookup: &mut HashMap<String, String>) {
        let words = vec![
            ("hai", "ہے"), ("hain", "ہیں"), ("tha", "تھا"), ("thay", "تھے"),
            ("thi", "تھی"), ("thin", "تھیں"), ("ho", "ہو"), ("huwa", "ہوا"),
            ("hoyega", "ہوگا"), ("ho gi", "ہو گی"), ("ho gay", "ہوئے"),
            ("ka", "کا"), ("ki", "کی"), ("ke", "کے"), ("ko", "کو"),
            ("se", "سے"), ("mein", "میں"), ("main", "میں"), ("neeche", "نیچے"),
            ("uper", "اوپر"), ("ander", "اندر"), ("bahar", "باہر"),
            ("aur", "اور"), ("yeh", "یہ"), ("ye", "یہ"), ("woh", "وہ"),
            ("wo", "وہ"), ("us", "اس"), ("in", "ان"), ("un", "ان"),
            ("is", "اس"), ("mera", "میرا"), ("meri", "میری"), ("mere", "میرے"),
            ("tera", "تیرا"), ("teri", "تیری"), ("tere", "تیرے"),
            ("apna", "اپنا"), ("apni", "اپنی"), ("apne", "اپنے"),
            ("hum", "ہم"), ("tum", "تم"), ("aap", "آپ"), ("usne", "اس نے"),
            ("unhone", "انہوں نے"), ("tumne", "تم نے"), ("aapne", "آپ نے"),
            ("kya", "کیا"), ("kyun", "کیوں"), ("kahan", "کہاں"),
            ("kaise", "کیسے"), ("kab", "کب"), ("kitna", "کتنا"),
            ("kitne", "کتنی"), ("kis", "کس"), ("kaun", "کون"),
            ("acha", "اچھا"), ("ache", "اچھے"), ("acchi", "اچھی"),
            ("theek", "ٹھیک"), ("bura", "برا"), ("bure", "برے"),
            ("nahi", "نہیں"), ("haan", "ہاں"), ("ji", "جی"),
            ("sahi", "صحیح"), ("ghalat", "غلط"), ("bat", "بات"),
            ("kam", "کام"), ("kam", "کم"), ("zyada", "زیادہ"),
            ("thoda", "تھوڑا"), ("thodi", "تھوڑی"), ("bohat", "بہت"),
            ("chahiye", "چاہیے"), ("chahiye", "چاہئے"),
            ("kar", "کر"), ("karo", "کرو"), ("karta", "کرتا"),
            ("karti", "کرتی"), ("karte", "کرتے"), ("karta", "کرتا"),
            ("ho", "ہو"), ("ja", "جا"), ("jao", "جاؤ"), ("jata", "جاتا"),
            ("jati", "جاتی"), ("jate", "جاتے"), ("aa", "آ"), ("ao", "آؤ"),
            ("aata", "آتا"), ("aati", "آتی"), ("aate", "آتے"),
            ("de", "دے"), ("do", "دو"), ("deta", "دیتا"), ("deti", "دیتی"),
            ("lete", "دیتے"), ("le", "لے"), ("lo", "لو"), ("leta", "لیتا"),
            ("leti", "لیتی"), ("rakh", "رکھ"), ("rakho", "رکھو"),
            ("rakhta", "رکھتا"), ("rakhti", "رکھتی"),
            ("sakta", "سکتا"), ("sakti", "سکتی"), ("sakte", "سکتے"),
            ("saktay", "سکتے"), ("chahiye", "چاہیے"),
            ("hoga", "ہوگا"), ("hogay", "ہوگے"), ("hogi", "ہوگی"),
            ("par", "پر"), ("pe", "پہ"), ("tak", "تک"),
            ("saath", "ساتھ"), ("liye", "لئے"), ("waste", "واسطے"),
            ("khatir", "خاطر"), ("lekin", "لیکن"), ("magar", "مگر"),
            ("agar", "اگر"), ("toh", "تو"), ("to", "تو"),
            ("bhi", "بھی"), ("hi", "ہی"), ("hee", "ہی"),
            ("sirf", "صرف"), ("bas", "بس"), ("phir", "پھر"),
            ("ab", "اب"), ("tab", "تب"), ("jab", "جب"),
            ("aisa", "ایسا"), ("aise", "ایسے"), ("waisa", "ویسا"),
            ("kyonke", "کیونکہ"), ("iss liye", "اس لیے"),
            ("jese", "جیسے"), ("wese", "ویسے"),
            ("ya", "یا"), ("lekin", "لیکن"),
            ("bahut", "بہت"), ("baray", "بڑے"), ("chotay", "چھوٹے"),
            ("namaz", "نماز"), ("roza", "روزہ"), ("zakat", "زکوٰۃ"),
            ("hajj", "حج"), ("quran", "قرآن"), ("allah", "اللہ"),
            ("pakistan", "پاکستان"), ("islamabad", "اسلام\u{200c}آباد"),
            ("lahore", "لاہور"), ("karachi", "کراچی"),
            ("urdu", "اردو"), ("punjabi", "پنجابی"),
            ("sindhi", "سندھی"), ("pashto", "پشتو"), ("balochi", "بلوچی"),
            ("assalam-o-alaikum", "السلام علیکم"),
            ("walaikum assalam", "وعلیکم السلام"),
            ("salam", "سلام"), ("adaab", "آداب"),
            ("shukriya", "شکریہ"), ("meherbani", "مہربانی"),
            ("afsoos", "افسوس"), ("mubarak", "مبارک"),
            ("dil", "دل"), ("pyar", "پیار"), ("mohabbat", "محبت"),
            ("dost", "دوست"), ("yar", "یار"), ("bhai", "بھائی"),
            ("behen", "بہن"), ("maa", "ماں"), ("baap", "باپ"),
            ("beta", "بیٹا"), ("beti", "بیٹی"),
        ];
        for (roman, urdu) in words {
            lookup.insert(roman.to_string(), urdu.to_string());
        }
    }
}
