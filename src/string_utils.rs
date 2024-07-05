use std::collections::{HashMap, HashSet};

pub fn compress_string(s: &str) -> String {
    let mut compressed = String::new();
    let mut count = 0;
    for (i, c) in s.chars().enumerate() {
        count += 1;
        if i + 1 >= s.len() || c != s.chars().nth(i + 1).unwrap() {
            compressed.push(c);
            compressed.push_str(&count.to_string());
            count = 0;
        }
    }
    if compressed.len() >= s.len() {
        s.to_string()
    } else {
        compressed
    }
}

pub fn can_construct(ranson_note: &str, magazine: &str) -> bool {
    let mut magazine_chars = HashMap::new();
    
    for ch in magazine.chars() {
        *magazine_chars.entry(ch).or_insert(0) += 1;
    }

    for ch in ranson_note.chars() {
        if let Some(count) = magazine_chars.get_mut(&ch) {
          if *count == 0 {
            return false;
          }
          *count -= 1;
        } else {
            return false
        }
    }
    true
}

pub fn word_pattern(pattern: &str, str_value: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let words: Vec<&str> = str_value.split_whitespace().collect();

    if pattern_chars.len() != words.len() {
        return false
    }

    let mut char_to_word = HashMap::new();
    let mut word_to_char = HashMap::new();
    let mut used_words = HashSet::new();

    for (i, &ch) in pattern_chars.iter().enumerate() {
        match (char_to_word.get(&ch), word_to_char.get(&words[i])) {
            (Some(&word), Some(&character)) => {
                if word != words[i] || character != ch {
                    return false;
                }
            },
            (None, None) => {
                char_to_word.insert(ch, words[i]);
                word_to_char.insert(words[i], ch);
                used_words.insert(words[i]);
            },
            _ => {return false;}
        }
    }
    char_to_word.len() == used_words.len() && word_to_char.len() == used_words.len()
}

pub fn conta_maiusculas(text: &str) -> u32 {
    let mut contador = 0;
    for caractere in text.chars() {
        if caractere.is_uppercase() {
            contador += 1;
        }
    }
    contador
}