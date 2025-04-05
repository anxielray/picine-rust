pub fn capitalize_first(input: &str) -> String {
    if let Some(first) = input.chars().next() {
        let c = first.to_ascii_uppercase().to_string();
        let r = input
            .chars()
            .skip(1)
            .collect::<String>()
            .to_ascii_lowercase();
        format!("{}{}", c, r)
    } else {
        String::new()
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut new_word = true;
    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            new_word = true;
        } else if new_word {
            result.push(c.to_ascii_uppercase());
            new_word = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        })
        .collect()
}
