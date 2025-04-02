pub fn first_subword(mut s: String) -> String{
    let mut result = String::new();
    let mut is_first = true;
    for c in s.chars() {
        if is_first {
            if c.is_alphanumeric() {
                result.push(c);
                is_first = false;
            }
        }else {
            if c == '_' || c == '+'{
                continue;
            }else if c.is_alphanumeric() {
                result.push(c);
            }else {
                break;
            }
        }
    }
    result
}
