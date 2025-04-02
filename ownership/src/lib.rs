pub fn first_subword(mut s: String) -> String{
    if let Some(pos) = s.find(|c: char| c == '_'|| c.is_uppercase()) {
        s.truncate(pos);
    }
    s
}
