use std::collections::HashMap;

static LATIN_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn is_pangram(phrase: &str) -> bool {
    let mut phrase_letters = HashMap::new();
    for ch in phrase.to_lowercase().chars() {
        phrase_letters.insert(ch, true);
    }

    LATIN_ALPHABET.chars()
        .all(|latin_letter| phrase_letters.contains_key(&latin_letter))
}
