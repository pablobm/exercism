use std::collections::HashMap;

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    let words = phrase
        .to_lowercase()
        .split(is_separator)
        .filter(|word| !word.is_empty() )
        .map(|word| word.to_string())
        .collect::<Vec<String>>();

    for word in words {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    counts
}

fn is_word_character(ch: char) -> bool {
    ch.is_alphanumeric()
}

fn is_separator(ch: char) -> bool {
    ! is_word_character(ch)
}