use dashmap::DashSet;
use regex::Regex;

pub struct Nysiis {
    vowels: DashSet<char>,
}

impl Nysiis {
    pub fn new() -> Self {
        let vowels = DashSet::new();
        vowels.insert('A');
        vowels.insert('E');
        vowels.insert('I');
        vowels.insert('O');
        vowels.insert('U');

        Nysiis { vowels }
    }

    pub fn encode(&self, name: &str) -> String {
        if name.is_empty() {
            return String::new();
        }

        let name = self.preprocess_name(name);

        if name.len() < 2 {
            return name;
        }

        let name = self.translate_first_characters(&name);
        let name = self.translate_last_characters(&name);
        let key = self.generate_key(&name);

        key
    }

    fn preprocess_name(&self, name: &str) -> String {
        let name = name.to_uppercase();
        let re = Regex::new(r"[^A-Z]").unwrap();
        re.replace_all(&name, "").to_string()
    }

    fn translate_first_characters(&self, name: &str) -> String {
        if name.starts_with("MAC") {
            return format!("MCC{}", &name[3..]);
        } else if name.starts_with("KN") {
            return format!("NN{}", &name[2..]);
        } else if name.starts_with("K") {
            return format!("C{}", &name[1..]);
        } else if name.starts_with("PH") || name.starts_with("PF") {
            return format!("FF{}", &name[2..]);
        } else if name.starts_with("SCH") {
            return format!("SSS{}", &name[3..]);
        } else if name.starts_with("GB") {
            return format!("J{}", &name[2..]);
        } else if name.starts_with("KP") {
            return format!("P{}", &name[2..]);
        } else if name.starts_with("NW") {
            return format!("W{}", &name[2..]);
        } else if name.starts_with("TS") {
            return format!("S{}", &name[2..]);
        } else if name.starts_with("SH") {
            return format!("S{}", &name[2..]);
        } else if name.starts_with("BH") {
            return format!("B{}", &name[2..]);
        } else if name.starts_with("DH") {
            return format!("D{}", &name[2..]);
        } else if name.starts_with("GH") {
            return format!("G{}", &name[2..]);
        } else if name.starts_with("JH") {
            return format!("J{}", &name[2..]);
        } else if name.starts_with("KH") {
            return format!("K{}", &name[2..]);
        } else if name.starts_with("PH") {
            return format!("F{}", &name[2..]);
        } else if name.starts_with("TH") {
            return format!("T{}", &name[2..]);
        } else if name.starts_with("CH") {
            return format!("C{}", &name[2..]);
        } else if name.starts_with("ZH") {
            return format!("J{}", &name[2..]);
        }

        name.to_string()
    }

    fn translate_last_characters(&self, name: &str) -> String {
        if name.ends_with("EE") || name.ends_with("IE") {
            return format!("{}Y", &name[..name.len() - 2]);
        } else if name.ends_with("DT")
            || name.ends_with("RT")
            || name.ends_with("RD")
            || name.ends_with("NT")
            || name.ends_with("ND")
        {
            return format!("{}D", &name[..name.len() - 2]);
        }

        name.to_string()
    }

    fn generate_key(&self, name: &str) -> String {
        let name_chars: Vec<char> = name.chars().collect();
        let mut key = String::new();
        key.push(name_chars[0]);

        let mut prev_char = name_chars[0];

        for i in 1..name_chars.len() {
            let mut current_char = name_chars[i];

            if self.vowels.contains(&current_char) {
                current_char = 'A';
            }

            current_char = self.translate_char(current_char, &name_chars, i);
            current_char = self.handle_vowel_harmony(current_char, prev_char);
            current_char = self.ignore_tonal_differences(current_char);

            if current_char != prev_char {
                key.push(current_char);
            }

            prev_char = current_char;
        }

        let key = self.remove_trailing_s(&key);
        let key = self.translate_ay(&key);
        let key = self.remove_trailing_a(&key);
        let key = self.truncate_key(&key);

        key
    }

    fn translate_char(&self, c: char, name: &[char], i: usize) -> char {
        if c == 'E' && i + 1 < name.len() && name[i + 1] == 'V' {
            return 'A';
        } else if c == 'Q' {
            return 'G';
        } else if c == 'Z' {
            return 'S';
        } else if c == 'M' {
            return 'N';
        } else if c == 'K' {
            if i + 1 < name.len() && name[i + 1] == 'N' {
                return name[i];
            } else {
                return 'C';
            }
        } else if c == 'S'
            && i + 2 < name.len()
            && name[i] == 'S'
            && name[i + 1] == 'C'
            && name[i + 2] == 'H'
        {
            return 'S';
        } else if c == 'P' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'F';
        } else if c == 'H'
            && (i == 0
                || i + 1 == name.len()
                || !self.vowels.contains(&name[i - 1])
                || !self.vowels.contains(&name[i + 1]))
        {
            return name[i - 1];
        } else if c == 'W' && i > 0 && self.vowels.contains(&name[i - 1]) {
            return name[i - 1];
        } else if c == 'G' && i + 1 < name.len() && name[i + 1] == 'B' {
            return 'J';
        } else if c == 'K' && i + 1 < name.len() && name[i + 1] == 'P' {
            return 'P';
        } else if c == 'N' && i + 1 < name.len() && name[i + 1] == 'W' {
            return 'W';
        } else if c == 'T' && i + 1 < name.len() && name[i + 1] == 'S' {
            return 'S';
        } else if c == 'S' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'S';
        } else if c == 'B' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'B';
        } else if c == 'D' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'D';
        } else if c == 'G' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'G';
        } else if c == 'J' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'J';
        } else if c == 'K' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'K';
        } else if c == 'P' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'F';
        } else if c == 'T' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'T';
        } else if c == 'C' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'C';
        } else if c == 'Z' && i + 1 < name.len() && name[i + 1] == 'H' {
            return 'J';
        }

        c
    }

    fn handle_vowel_harmony(&self, c: char, prev: char) -> char {
        if self.vowels.contains(&c) && self.vowels.contains(&prev) {
            if prev == 'A' || prev == 'O' || prev == 'U' {
                if c == 'E' || c == 'I' {
                    return 'A';
                }
            } else if prev == 'E' || prev == 'I' {
                if c == 'A' || c == 'O' || c == 'U' {
                    return 'E';
                }
            }
        }
        c
    }

    fn ignore_tonal_differences(&self, c: char) -> char {
        if c >= 'A' && c <= 'Z' {
            c.to_ascii_uppercase()
        } else {
            c
        }
    }

    fn remove_trailing_s(&self, key: &str) -> String {
        if key.len() > 1 && key.ends_with('S') {
            key[..key.len() - 1].to_string()
        } else {
            key.to_string()
        }
    }

    fn translate_ay(&self, key: &str) -> String {
        if key.ends_with("AY") {
            format!("{}Y", &key[..key.len() - 2])
        } else {
            key.to_string()
        }
    }

    fn remove_trailing_a(&self, key: &str) -> String {
        if key.len() > 1 && key.ends_with('A') {
            key[..key.len() - 1].to_string()
        } else {
            key.to_string()
        }
    }

    fn truncate_key(&self, key: &str) -> String {
        if key.len() > 6 {
            key[..6].to_string()
        } else {
            key.to_string()
        }
    }
}
