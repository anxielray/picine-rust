pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    
    words.sort_by_key(|word| {
        word.chars()
            .filter_map(|c| c.to_digit(10))
            .next()
            .unwrap_or(0)
    });
    
    words.join(" ")
}
