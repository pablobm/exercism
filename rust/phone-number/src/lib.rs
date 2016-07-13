struct PhoneNumber {
    country: char,
    area: Box<[char; 3]>,
    exchange: Box<[char; 3]>,
    subscriber: Box<[char; 4]>,
}

impl PhoneNumber {
    pub fn is_valid(&self) -> bool {
        self.area.iter().all(|&c| c != 'x') &&
            self.exchange.iter().all(|&c| c != 'x') &&
            self.subscriber.iter().all(|&c| c != 'x') &&
            self.country == '1'
    }
}

fn parse(number: &str) -> PhoneNumber {
    let mut tpl = PhoneNumber {
        country: '1',
        area: Box::new(['x'; 3]),
        exchange: Box::new(['x'; 3]),
        subscriber: Box::new(['x'; 4]),
    };
    let rev_numbers = number.chars()
        .filter(|n| n.is_numeric())
        .rev();

    for (i, n) in rev_numbers.enumerate() {
        match i {
            0...3 => tpl.subscriber[3-i] = n,
            4...6 => tpl.exchange[6-i] = n,
            7...9 => tpl.area[9-i] = n,
            10    => tpl.country = n,
            _     => {},
        };
    }

    tpl
}

pub fn number(input: &str) -> Option<String> {
    let phone_number = parse(input);

    if phone_number.is_valid() {
        let string = format!(
            "{}{}{}",
            phone_number.area.iter().cloned().collect::<String>(),
            phone_number.exchange.iter().cloned().collect::<String>(),
            phone_number.subscriber.iter().cloned().collect::<String>(),
        );
        Some(string)
    }
    else {
        None
    }
}

pub fn area_code(input: &str) -> Option<String> {
    let phone_number = parse(input);

    if phone_number.is_valid() {
        let string = format!(
            "{}",
            phone_number.area.iter().cloned().collect::<String>(),
        );
        Some(string)
    }
    else {
        None
    }
}

pub fn pretty_print(input: &str) -> String {
    let phone_number = parse(input);

    if phone_number.is_valid() {
        format!(
            "({}) {}-{}",
            phone_number.area.iter().cloned().collect::<String>(),
            phone_number.exchange.iter().cloned().collect::<String>(),
            phone_number.subscriber.iter().cloned().collect::<String>(),
        )
    }
    else {
        "invalid".to_string()
    }
}
