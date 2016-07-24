fn sanitize(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_numeric())
        .collect()
}

fn parse(input: String) -> Option<String> {
    if input.len() == 11 {
        if input.starts_with("1") {
            Some(input.chars().skip(1).take(10).collect())
        }
        else {
            None
        }
    }
    else if input.len() == 10 {
        Some(input.to_string())
    }
    else {
        None
    }
}

pub fn number(input: &str) -> Option<String> {
    parse(sanitize(input))
}

pub fn area_code(input: &str) -> Option<String> {
    number(input).map(|n| n[0..3].to_string())
}

fn exchange(input: &str) -> Option<String> {
    number(input).map(|n| n[3..6].to_string())
}

fn subscriber(input: &str) -> Option<String> {
    number(input).map(|n| n[6..10].to_string())
}

pub fn pretty_print(input: &str) -> String {
    match number(input) {
        Some(_) => format!(
            "({}) {}-{}",
            area_code(input).unwrap(),
            exchange(input).unwrap(),
            subscriber(input).unwrap(),
        ),
        None    => "invalid".to_string(),
    }
}
