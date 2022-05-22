pub fn hex_to_int(hex: &str) -> Option<u32> {
    hex.chars()
        .map(hex_digit_to_int)
        .rev()
        .enumerate()
        .fold(Some(0), {|maybe_acc, (idx, maybe_digit)|
            match (maybe_acc, maybe_digit) {
                (Some(acc), Some(digit))
                    => Some(acc + digit * (16 as u32).pow(idx as u32)),
                (_, _)
                    => None,
            }
        })
}

fn hex_digit_to_int(digit: char) -> Option<u32> {
    match digit {
        '0' => Some( 0),
        '1' => Some( 1),
        '2' => Some( 2),
        '3' => Some( 3),
        '4' => Some( 4),
        '5' => Some( 5),
        '6' => Some( 6),
        '7' => Some( 7),
        '8' => Some( 8),
        '9' => Some( 9),
        'a' => Some(10),
        'b' => Some(11),
        'c' => Some(12),
        'd' => Some(13),
        'e' => Some(14),
        'f' => Some(15),
        _   => None,
    }
}