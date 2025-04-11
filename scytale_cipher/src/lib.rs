fn scytale_cipher(message: String, size: u32) -> String {
    let message = message.replace(" ", "");
    let size = size as usize;
    let mut rows = vec![String::new(); (message.len() + size - 1) / size];
    let mut index = 0;

    // Fill the rows
    for i in 0..message.len() {
        rows[index].push(message.chars().nth(i).unwrap());
        index = (index + 1) % size;
    }

    // Concatenate the rows
    rows.join("")
}
