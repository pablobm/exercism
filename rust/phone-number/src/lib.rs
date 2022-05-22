use std::ops::Range;

const AREA_RANGE       : Range<usize> = 0..3;
const EXCHANGE_RANGE   : Range<usize> = 3..6;
const SUBSCRIBER_RANGE : Range<usize> = 6..10;

fn sanitize(input: &str) -> String {
    input.matches(char::is_numeric).collect()
}

fn parse(input: String) -> Option<String> {
    match input.len() {
        11 if input.starts_with("1") => Some(input[1..].to_string()),
        10                           => Some(input),
        _                            => None,
    }
}

pub fn number(input: &str) -> Option<String> {
    parse(sanitize(input))
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