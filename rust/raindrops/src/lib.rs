pub fn raindrops(n: i32) -> String {
    let sounds = (
        if n % 3 == 0 { "Pling" } else { "" },
        if n % 5 == 0 { "Plang" } else { "" },
        if n % 7 == 0 { "Plong" } else { "" },
    );
    match sounds {
        (a, b, c) if a == b && b == c && c == ""
            => n.to_string(),
        (a, b, c)
            => format!("{}{}{}", a, b, c),
    }
}
