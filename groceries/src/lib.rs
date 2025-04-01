pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    slice.get(index).map(|s| s.as_str()).expect("Index out of bounds.")
}
