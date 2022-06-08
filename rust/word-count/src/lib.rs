use std::collections::HashMap;

const SINGLE_QUOTE : char = '\'';

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    let words = phrase
        .to_lowercase()
        .split(is_separator)
        .filter(|word| !word.is_empty() )
        .map(|word| word.to_string())
        .map(remove_quotations)
        .collect::<Vec<String>>();

    for word in words {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    counts
}

fn remove_quotations(s: String) -> String {
    if s.len() < 2 {
        return s
    }

    let mut each_char = s.chars();

    if each_char.next() == Some(SINGLE_QUOTE) && each_char.last() == Some(SINGLE_QUOTE) {
        return s[1..=s.len()-2].to_string()
    }
    s
}

fn is_word_character(ch: char) -> bool {
    ch.is_alphanumeric() || ch == SINGLE_QUOTE
}

fn is_separator(ch: char) -> bool {
    ! is_word_character(ch)
}
