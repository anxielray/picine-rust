#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected_ciphered = atbash(original);
    if ciphered == expected_ciphered {
        Ok(())
    } else {
        Err(CipherError {
            expected: expected_ciphered,
        })
    }
}

fn atbash(input: &str) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            (base + (b'Z' - c.to_ascii_uppercase() as u8)) as char
        } else {
            c
        }
    }).collect()
}

