pub fn reply(input: &str) -> &str {
    let yelling = input.to_uppercase() == input;

    if input == "" {
        "Fine. Be that way!"
    }
    else if yelling {
        "Whoa, chill out!"
    }
    else if input.ends_with('?') {
        "Sure."
    }
    else {
        "Whatever."
    }
}
