pub fn scytale_cipher(message: String, size: u32) -> String {
    let size = size as usize;
    let chars: Vec<char> = message.chars().collect();
    let length = chars.len();

    let mut result = String::new();

    for col in 0..size {
        let mut idx = col;
        while idx < length {
            result.push(chars[idx]);
            idx += size;
        }
    }

    result
}
