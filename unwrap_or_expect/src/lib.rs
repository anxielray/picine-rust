pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

// This is already internally defined by the rust compiler, no need to include it here
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match server {
        Ok(url) => {
            match security_level {
                Security::Unknown => url.to_string(),
                Security::Message => url.to_string(),
                Security::Warning => url.to_string(),
                Security::NotFound => url.to_string(),
                Security::UnexpectedUrl => url.to_string(),
            }
        }
        Err(error_message) => {
            match security_level {
                Security::Unknown => panic!("{}", error_message),
                Security::Message => panic!("ERROR: program stops"),
                Security::Warning => format!("WARNING: check the server"),
                Security::NotFound => format!("Not found: {}", error_message),
                Security::UnexpectedUrl => panic!("{}", error_message),
            }
        }
    }
}
