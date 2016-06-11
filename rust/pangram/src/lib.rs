use std::collections::HashMap;

static LATIN_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn is_pangram(phrase: &str) -> bool {
    let phrase_letters = phrase.to_lowercase().chars()
        .map(|x| (x, true))
        .collect::<HashMap<_, _>>();

    LATIN_ALPHABET.chars()
        .all(|latin_letter| phrase_letters.contains_key(&latin_letter))
}
