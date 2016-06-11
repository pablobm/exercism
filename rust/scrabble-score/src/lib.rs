pub fn score(word: &str) -> u32 {
    word.chars()
        .map(|ch| letter_score(ch) )
        .fold(0, |acc, ch| acc + ch)
}

fn letter_score(letter: char) -> u32 {
    match letter.to_lowercase().next().unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' |
        'l' | 'n' | 'r' | 's' | 't'
            => 1,
        'd' | 'g'
            => 2,
        'b' | 'c' | 'm' | 'p'
            => 3,
        'f' | 'h' | 'v' | 'w' | 'y'
            => 4,
        'k'
            => 5,
        'j' | 'x'
            => 8,
        'q' | 'z'
            => 10,
        _
            => 0,
    }
}
