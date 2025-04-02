pub fn initals(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_whitespace()
                .map(|word| format!("{}.", word.chars().nex().unwrap()))
                .collect::<Vec<String>>()
                .join("")
        })
        .collect()
}
