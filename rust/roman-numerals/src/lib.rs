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
        sections.push_front(to_units_string(quotient % 10));

        quotient = quotient / 10;
        if quotient > 0 {
            sections.push_front(to_tens_string(quotient % 10))
        }

        quotient = quotient / 10;
        if quotient > 0 {
            sections.push_front(to_hundreds_string(quotient % 10))
        }

        quotient = quotient / 10;
        if quotient > 0 {
            sections.push_front(to_thousands_string(quotient % 10))
        }

        sections.iter().map(|x| x.clone() ).collect()
    }

}

fn to_units_string(value: u16) -> String {
    match value {
        0 => "",
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        _ => panic!("I don't know what to do with {}", value)
    }.to_string()
}

fn to_tens_string(value: u16) -> String {
    match value {
        0 => "",
        1 => "X",
        2 => "XX",
        3 => "XXX",
        4 => "XL",
        5 => "L",
        6 => "LX",
        7 => "LXX",
        8 => "LXXX",
        9 => "XC",
        _ => panic!("I don't know what to do with {}", value)
    }.to_string()
}

fn to_hundreds_string(value: u16) -> String {
    match value {
        0 => "",
        1 => "C",
        2 => "CC",
        3 => "CCC",
        4 => "CD",
        5 => "D",
        6 => "DC",
        7 => "DCC",
        8 => "DCCC",
        9 => "CM",
        _ => panic!("I don't know what to do with {}", value)
    }.to_string()
}

fn to_thousands_string(value: u16) -> String {
    match value {
        0 => "",
        1 => "M",
        2 => "MM",
        3 => "MMM",
        _ => panic!("I don't know what to do with {}", value)
    }.to_string()
}
