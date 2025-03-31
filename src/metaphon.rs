pub struct Metaphone {
    vowels: Vec<char>,
    iey: Vec<char>,
}

impl Metaphone {
    pub fn new() -> Self {
        Metaphone {
            vowels: vec!['a', 'e', 'i', 'o', 'u'],
            iey: vec!['i', 'e', 'y'],
        }
    }

    pub fn encode(&self, val: &str) -> Result<String, String> {
        if val.is_empty() || !self.is_alphabetic(val) {
            return Err("String is empty or non-alphabetic.".to_string());
        }

        let deduplicated = self.de_duplicate(&val.to_lowercase());
        let first_char_processed = self.transcode_first_character(&deduplicated);
        let transcoded = self.transcode(&first_char_processed);

        Ok(transcoded)
    }

    pub fn compare(&self, val1: &str, val2: &str) -> Result<bool, String> {
        let s1_is_alphabetic = self.is_alphabetic(val1);
        let s2_is_alphabetic = self.is_alphabetic(val2);

        if val1.is_empty() || !s1_is_alphabetic || val2.is_empty() || !s2_is_alphabetic {
            return Err("Unable to Metaphone compare the two values.".to_string());
        }

        let phonetic1 = self.encode(val1)?;
        let phonetic2 = self.encode(val2)?;

        Ok(phonetic1 == phonetic2)
    }

    fn is_alphabetic(&self, val: &str) -> bool {
        val.chars().all(|c| c.is_alphabetic())
    }

    pub fn de_duplicate(&self, val: &str) -> String {
        let mut result = String::with_capacity(val.len());
        let mut chars = val.chars().peekable();

        while let Some(c) = chars.next() {
            if chars.peek() != Some(&c) {
                result.push(c);
            }
        }

        result
    }

    fn transcode_first_character(&self, s: &str) -> String {
        let chars: Vec<char> = s.chars().collect();

        match chars.len() {
            0 => String::new(),
            1 => {
                let first_letter = chars[0];
                match first_letter {
                    'x' => "s".to_string(),
                    _ => s.to_string(),
                }
            }
            _ => {
                let letter1 = chars[0];
                let letter2 = chars[1];
                let rest_of_word1: String = chars[1..].iter().collect();
                let rest_of_word2: String = if chars.len() > 2 {
                    chars[2..].iter().collect()
                } else {
                    String::new()
                };

                match letter1 {
                    'a' => match letter2 {
                        'e' => rest_of_word1,
                        _ => s.to_string(),
                    },
                    'g' | 'k' | 'p' => match letter2 {
                        'n' => rest_of_word1,
                        _ => s.to_string(),
                    },
                    'w' => match letter2 {
                        'r' => rest_of_word1,
                        'h' => format!("w{}", rest_of_word2),
                        _ => s.to_string(),
                    },
                    'x' => format!("s{}", rest_of_word1),
                    _ => s.to_string(),
                }
            }
        }
    }

    fn transcode(&self, s: &str) -> String {
        let characters: Vec<char> = s.chars().collect();
        let size = characters.len();

        let mut p_count = 0;
        let mut output = vec![' '; size * 2];
        let mut o_count = 0;

        while p_count < size {
            let current_char = characters[p_count];
            let r_size = if p_count + 1 >= size {
                0
            } else {
                size - (p_count + 1)
            };

            match current_char {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    if p_count == 0 {
                        output[o_count] = current_char;
                        o_count += 1;
                    }
                    p_count += 1;
                }
                'f' | 'j' | 'l' | 'm' | 'n' | 'r' => {
                    output[o_count] = current_char;
                    o_count += 1;
                    p_count += 1;
                }
                'b' => {
                    if p_count >= 1 && p_count + 1 >= size && characters[p_count - 1] == 'm' {
                        p_count += 1;
                    } else {
                        output[o_count] = 'b';
                        o_count += 1;
                        p_count += 1;
                    }
                }
                'c' => {
                    if r_size >= 1
                        && characters[p_count + 1] == 'h'
                        && p_count >= 1
                        && characters[p_count - 1] == 's'
                    {
                        output[o_count] = 'k';
                        o_count += 1;
                        p_count += 1;
                    } else if r_size >= 2
                        && characters[p_count + 1] == 'i'
                        && characters[p_count + 2] == 'a'
                    {
                        output[o_count] = 'x';
                        o_count += 1;
                        p_count += 3;
                    } else if (r_size >= 1 && characters[p_count + 1] == 'h')
                        || (p_count >= 1
                            && r_size >= 1
                            && characters[p_count - 1] == 's'
                            && characters[p_count + 1] == 'h')
                    {
                        output[o_count] = 'x';
                        o_count += 1;
                        p_count += 2;
                    } else if p_count >= 1
                        && r_size >= 1
                        && characters[p_count - 1] == 's'
                        && self.iey.contains(&characters[p_count + 1])
                    {
                        p_count += 1;
                    } else if r_size >= 1 && self.iey.contains(&characters[p_count + 1]) {
                        output[o_count] = 's';
                        o_count += 1;
                        p_count += 1;
                    } else {
                        output[o_count] = 'k';
                        o_count += 1;
                        p_count += 1;
                    }
                }
                'd' => {
                    if r_size >= 2
                        && characters[p_count + 1] == 'g'
                        && self.iey.contains(&characters[p_count + 2])
                    {
                        output[o_count] = 'j';
                        o_count += 1;
                        p_count += 1;
                    } else {
                        output[o_count] = 't';
                        o_count += 1;
                        p_count += 1;
                    }
                }
                'g' => {
                    if (r_size > 1 && characters[p_count + 1] == 'h')
                        || (r_size == 1 && characters[p_count + 1] == 'n')
                        || (r_size == 3
                            && characters[p_count + 1] == 'n'
                            && characters[p_count + 3] == 'd')
                    {
                        p_count += 1;
                    } else if r_size >= 1 && self.iey.contains(&characters[p_count + 1]) {
                        output[o_count] = 'j';
                        o_count += 1;
                        p_count += 2;
                    } else {
                        output[o_count] = 'k';
                        o_count += 1;
                        p_count += 1;
                    }
                }
                'h' => {
                    if (p_count >= 1
                        && self.vowels.contains(&characters[p_count - 1])
                        && (r_size == 0 || self.vowels.contains(&characters[p_count + 1])))
                        || (p_count >= 2
                            && characters[p_count - 1] == 'h'
                            && (characters[p_count - 2] == 't' || characters[p_count - 2] == 'g'))
                    {
                        p_count += 1;
                    } else {
                        output[o_count] = 'h';
                        o_count += 1;
                        p_count += 1;
                    }
                }
                'k' => {
                    if p_count >= 1 && characters[p_count - 1] == 'c' {
                        p_count += 1;
                    } else {
                        output[o_count] = 'k';
                        o_count += 1;
                        p_count += 1;
                    }
                }
                'p' => {
                    if r_size >= 1 && characters[p_count + 1] == 'h' {
                        output[o_count] = 'f';
                        o_count += 1;
                        p_count += 2;
                    } else {
                        output[o_count] = 'p';
                        o_count += 1;
                        p_count += 1;
                    }
                }
                'q' => {
                    output[o_count] = 'k';
                    o_count += 1;
                    p_count += 1;
                }
                's' => {
                    if r_size >= 2
                        && characters[p_count + 1] == 'i'
                        && ['a', 'o'].contains(&characters[p_count + 2])
                    {
                        output[o_count] = 'x';
                        o_count += 1;
                        p_count += 3;
                    } else if r_size >= 1 && characters[p_count + 1] == 'h' {
                        output[o_count] = 'x';
                        o_count += 1;
                        p_count += 2;
                    } else {
                        output[o_count] = 's';
                        o_count += 1;
                        p_count += 1;
                    }
                }
                't' => {
                    if r_size >= 2
                        && characters[p_count + 1] == 'i'
                        && ['a', 'o'].contains(&characters[p_count + 2])
                    {
                        output[o_count] = 'x';
                        o_count += 1;
                        p_count += 3;
                    } else if r_size >= 1 && characters[p_count + 1] == 'h' {
                        output[o_count] = '0';
                        o_count += 1;
                        p_count += 2;
                    } else if r_size >= 2
                        && characters[p_count + 1] == 'c'
                        && characters[p_count + 2] == 'h'
                    {
                        p_count += 1;
                    } else {
                        output[o_count] = 't';
                        o_count += 1;
                        p_count += 1;
                    }
                }
                'v' => {
                    output[o_count] = 'f';
                    o_count += 1;
                    p_count += 1;
                }
                'w' | 'y' => {
                    if r_size == 0 || !self.vowels.contains(&characters[p_count + 1]) {
                        p_count += 1;
                    } else {
                        output[o_count] = current_char;
                        o_count += 1;
                        p_count += 1;
                    }
                }
                'x' => {
                    output[o_count] = 'k';
                    output[o_count + 1] = 's';
                    o_count += 2;
                    p_count += 1;
                }
                'z' => {
                    output[o_count] = 's';
                    o_count += 1;
                    p_count += 1;
                }
                _ => {
                    p_count += 1;
                }
            }
        }
        output[0..o_count].iter().collect()
    }
}

impl Default for Metaphone {
    fn default() -> Self {
        Self::new()
    }
}

pub mod metaphone {
    use super::Metaphone;
    pub fn metaphone(val: &str) -> Result<String, String> {
        Metaphone::new().encode(val)
    }

    pub fn metaphone_metric(val1: &str, val2: &str) -> Result<bool, String> {
        Metaphone::new().compare(val1, val2)
    }
}
