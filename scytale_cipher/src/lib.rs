pub fn scytale_cipher(message: String, size: u32) -> String {
    let size = size as usize; // Convert `size` to `usize` for indexing
    let chars: Vec<char> = message.chars().collect(); // Convert message to a vector of characters
    let length = chars.len();

    // Create a result string with enough space for the final cipher
    let mut result = String::new();

    // Iterate column by column
    for col in 0..size {
        let mut idx = col;
        while idx < length {
            result.push(chars[idx]); // Add character to result
            idx += size; // Move to the next row in the same column
        }
    }

    result
}