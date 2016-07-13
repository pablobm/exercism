use std::collections::LinkedList;

struct PhoneNumber {
    country: char,
    area: LinkedList<char>,
    exchange: LinkedList<char>,
    subscriber: LinkedList<char>,
}

impl PhoneNumber {
    pub fn new(number: &str) -> PhoneNumber {
        let mut tpl = PhoneNumber {
            country: '1',
            area: LinkedList::new(),
            exchange: LinkedList::new(),
            subscriber: LinkedList::new(),
        };
        let numbers = number.chars()
            .filter(|n| n.is_numeric());

        for (i, n) in numbers.rev().enumerate() {
            match i {
                0...3 => tpl.subscriber.push_front(n),
                4...6 => tpl.exchange.push_front(n),
                7...9 => tpl.area.push_front(n),
                10    => tpl.country = n,
                _     => {},
            };
        }

        tpl
    }

    pub fn is_valid(&self) -> bool {
        self.area.len() == 3 &&
            self.exchange.len() == 3 &&
            self.subscriber.len() == 4 &&
            self.country == '1'
    }

    pub fn area(&self) -> String {
        stringify(&self.area)
    }

    pub fn exchange(&self) -> String {
        stringify(&self.exchange)
    }

    pub fn subscriber(&self) -> String {
        stringify(&self.subscriber)
    }
}

fn stringify(list: &LinkedList<char>) -> String {
    list.iter().cloned().collect::<String>()
}

pub fn number(input: &str) -> Option<String> {
    let phone_number = PhoneNumber::new(input);

    if phone_number.is_valid() {
        let string = format!(
            "{}{}{}",
            phone_number.area(),
            phone_number.exchange(),
            phone_number.subscriber(),
        );
        Some(string)
    }
    else {
        None
    }
}

pub fn area_code(input: &str) -> Option<String> {
    let phone_number = PhoneNumber::new(input);

    if phone_number.is_valid() {
        Some(phone_number.area())
    }
    else {
        None
    }
}

pub fn pretty_print(input: &str) -> String {
    let phone_number = PhoneNumber::new(input);

    if phone_number.is_valid() {
        format!(
            "({}) {}-{}",
            phone_number.area(),
            phone_number.exchange(),
            phone_number.subscriber(),
        )
    }
    else {
        "invalid".to_string()
    }
}
