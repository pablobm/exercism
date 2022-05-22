pub fn reply(input: &str) -> &str {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        "Fine. Be that way!"
    }
    else if has_letters(trimmed) && trimmed.to_uppercase() == trimmed {
        if trimmed.ends_with("?") {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    }
    else if trimmed.ends_with('?') {
        "Sure."
    }
    else {
        "Whatever."
    }
}

fn has_letters(input: &str) -> bool {
    input.matches(char::is_alphabetic).count() != 0
}
