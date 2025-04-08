pub struct Message {
    content: String,
    user: String,
}

impl Message {
    // Constructor to initialize a new Message
    pub fn new(content: String, user: String) -> Self {
        Message { content, user }
    }

    // Method to check if the message is valid
    pub fn send_ms(self) -> Option<String> {
        // If the message is empty or contains the word "stupid", return None
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(self.content) // Return the content if it's valid
        }
    }
}

// Function to check the message and return the appropriate Result
pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message.to_string(), "user".to_string());
    
    match msg.send_ms() {
        Some(content) => Ok(content.as_str()), // If the message is valid, return Ok with the content
        None => Err("ERROR: illegal"), // If the message is invalid, return an error
    }
}
