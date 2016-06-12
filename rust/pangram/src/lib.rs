use std::collections::HashSet;

static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn is_pangram(phrase: &str) -> bool {
    let phrase_letters = phrase.to_lowercase().chars()
        .collect::<HashSet<_>>();

    ALPHABET.chars()
        .all(|alphabet_letter| phrase_letters.contains(&alphabet_letter))
}
