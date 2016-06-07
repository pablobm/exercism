pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(name_str) => format!("Hello, {}!", name_str),
        None => "Hello, World!".to_string(),
    }
}
