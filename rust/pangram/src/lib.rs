use std::collections::HashMap;

static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn is_pangram(phrase: &str) -> bool {
    let phrase_letters = phrase.to_lowercase().chars()
        .map(|x| (x, true))
        .collect::<HashMap<_, _>>();

    ALPHABET.chars()
        .all(|alphabet_letter| phrase_letters.contains_key(&alphabet_letter))
}
