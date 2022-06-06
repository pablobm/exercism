use std::ops::Range;

const AREA_RANGE       : Range<usize> = 0..3;
const EXCHANGE_RANGE   : Range<usize> = 3..6;
const SUBSCRIBER_RANGE : Range<usize> = 6..10;

fn clean_number(input: &str) -> Option<String> {
    let mut output = String::new();
    for c in input.chars() {
        match c {
            n if n.is_ascii_digit() => output.push(n),
            '+' | '-' | ' ' | '(' | ')' | '.' => (),
            _ => return None
        }
    }
    Some(output)
}

fn cut_to_length(input: String) -> Option<String> {
    if input.len() == 10 {
        Some(input)
    } else if input.len() == 11 && input.chars().next() == Some('1') {
        Some(input[1..11].to_string())
    } else {
        None
    }
}

fn ensure_valid_digits(input: String) -> Option<String> {
    for (i, c) in input.char_indices() {
        if (i == 0 || i == 3) && (c == '0' || c == '1') {
            return None
        }
    }
    Some(input)
}

pub fn number(input: &str) -> Option<String> {
    clean_number(input)
        .and_then(cut_to_length)
        .and_then(ensure_valid_digits)
}

pub fn area_code(input: &str) -> Option<String> {
    number(input).map(|n| n[AREA_RANGE].to_string())
}

pub fn pretty_print(input: &str) -> String {
    match number(input) {
        Some(n) => format!("({}) {}-{}", &n[AREA_RANGE], &n[EXCHANGE_RANGE], &n[SUBSCRIBER_RANGE]),
        None    => "invalid".to_string(),
    }
}
