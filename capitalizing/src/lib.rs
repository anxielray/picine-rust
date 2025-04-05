pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

use regex::Regex;

pub fn title_case(input: &str) -> String {
    let re = Regex::new(r"(\s+)").unwrap();
    re.split(input)
        .map(|part| {
            if part.trim().is_empty() {
                part.to_string()
            } else {
                capitalize_first(part)
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_uppercase().collect::<String>()
            }
        })
        .collect()
}
