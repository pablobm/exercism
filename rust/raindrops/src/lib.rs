use std::fmt;

enum DropSound<'a> {
    Name(&'a str),
    Number,
}

use self::DropSound::*;

impl<'a> fmt::Display for DropSound<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Name(name) => write!(f, "{}", name),
            Number => write!(f, ""),
        }
    }
}

pub fn raindrops(n: i32) -> String {
    let sounds = (
        if n % 3 == 0 { Name("Pling") } else { Number },
        if n % 5 == 0 { Name("Plang") } else { Number },
        if n % 7 == 0 { Name("Plong") } else { Number },
    );
    match sounds {
        (Number, Number, Number) => n.to_string(),
        (a, b, c)                => format!("{}{}{}", a, b, c),
    }
}
