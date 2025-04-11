pub fn encode(text: &str, key: i32) -> &str {
    let mut ciphered = String::new();
    let mut chars = text.chars();
    let k = text.len() as i32 / key;
    for i in 0..text.len() {
        let index = ((i as i32 % key) * k + i as i32 / key) as usize;
        println!("{}", index);
        let charachter = &chars.nth(index).unwrap();
        ciphered.push(*charachter);
    }

    ""
}
