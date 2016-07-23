struct PhoneNumber {
    number: Option<String>,
}

fn sanitize(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_numeric())
        .collect()
}

fn parse(input: String) -> Option<String> {
    if input.len() == 11 {
        if input.starts_with("1") {
            Some(input.chars().skip(1).take(10).collect())
        }
        else {
            None
        }
    }
    else if input.len() == 10 {
        Some(input.to_string())
    }
    else {
        None
    }
}

impl PhoneNumber {
    pub fn new(input: &str) -> PhoneNumber {
        PhoneNumber {
            number: parse(sanitize(input)),
        }
    }

    pub fn area(&self) -> Option<String> {
        self.number.clone().map(|n| n[0..3].to_string())
    }

    pub fn exchange(&self) -> Option<String> {
        self.number.clone().map(|n| n[3..6].to_string())
    }

    pub fn subscriber(&self) -> Option<String> {
        self.number.clone().map(|n| n[6..10].to_string())
    }
}

pub fn number(input: &str) -> Option<String> {
    PhoneNumber::new(input).number
}

pub fn area_code(input: &str) -> Option<String> {
    PhoneNumber::new(input).area()
}

pub fn pretty_print(input: &str) -> String {
    let phone_number = PhoneNumber::new(input);

    match phone_number.number {
        Some(_) => format!(
            "({}) {}-{}",
            phone_number.area().unwrap(),
            phone_number.exchange().unwrap(),
            phone_number.subscriber().unwrap(),
        ),
        None    => "invalid".to_string(),
    }
}
