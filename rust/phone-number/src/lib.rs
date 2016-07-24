fn sanitize(input: &str) -> String {
    input.matches(char::is_numeric).collect()
}

fn parse(input: String) -> Option<String> {
    match input.len() {
        11 if input.starts_with("1") => Some(input[1..].to_string()),
        11                           => None,
        10                           => Some(input),
        _                            => None,
    }
}

pub fn number(input: &str) -> Option<String> {
    parse(sanitize(input))
}

pub fn area_code(input: &str) -> Option<String> {
    number(input).map(|n| n[0..3].to_string())
}

pub fn pretty_print(input: &str) -> String {
    match number(input) {
        Some(n) => format!("({}) {}-{}", &n[0..3], &n[3..6], &n[6..10]),
        None    => "invalid".to_string(),
    }
}
