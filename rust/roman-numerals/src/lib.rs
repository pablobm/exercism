use std::collections::LinkedList;

pub struct Roman {
    value: u16,
}

// Not implementing `std::convert::From` because
// its contract specifies that it must not fail
impl Roman {
    pub fn from(value: u16) -> Roman {
        Roman {
            value: value,
        }
    }
}

impl ToString for Roman {
    fn to_string(&self) -> String {
        let mut sections = LinkedList::new();

        let mut quotient = self.value;
        sections.push_front(roman_string(quotient % 10, 'I', 'V', 'X'));

        quotient = quotient / 10;
        if quotient > 0 {
            sections.push_front(roman_string(quotient % 10, 'X', 'L', 'C'));
        }

        quotient = quotient / 10;
        if quotient > 0 {
            sections.push_front(roman_string(quotient % 10, 'C', 'D', 'M'));
        }

        quotient = quotient / 10;
        if quotient > 0 {
            sections.push_front(roman_string(quotient % 10, 'M', '!', '!'));
        }

        let mut result = String::new();
        for letter in sections {
            result.push_str(&letter);
        }
        result
    }
}

fn roman_string(digit: u16, one: char, five: char, ten: char) -> String {
    let digits = match digit {
        0 => vec![],
        1 => vec![one],
        2 => vec![one, one],
        3 => vec![one, one, one],
        4 => vec![one, five],
        5 => vec![five],
        6 => vec![five, one],
        7 => vec![five, one, one],
        8 => vec![five, one, one, one],
        9 => vec![one, ten],
        _ => panic!("I don't know what to do with {}", digit),
    };

    digits.iter().cloned().collect()
}