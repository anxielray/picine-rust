pub fn first_subword(mut s: String) -> String {
    let mut result = String::new();

    for (i, c) in s.chars().enumerate() {
        if c == '_' {
            break;
        }
        if i > 0 && c.is_uppercase() {
            break;
        }
        result.push(c);
    }

    result
}
