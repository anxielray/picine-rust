use std::{error::Error, fmt};

// Enum to represent parsing errors
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

// Implement Display for ParseErr
impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErr::Empty => write!(f, "Failed to parse todo file: None"),
            ParseErr::Malformed(err) => write!(f, "Failed to parse todo file: Some({})", err),
        }
    }
}

// Implement Error for ParseErr
impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Empty => None,
            ParseErr::Malformed(err) => Some(&**err),
        }
    }
}

// Struct to represent reading errors
pub struct ReadErr {
    child_err: Box<dyn Error>,
}

// Implement Display for ReadErr
impl fmt::Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to read todo file")
    }
}

// Implement Error for ReadErr
impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.child_err)
    }
}

// Implement From<Box<dyn Error>> for ReadErr
impl From<Box<dyn Error>> for ReadErr {
    fn from(err: Box<dyn Error>) -> Self {
        ReadErr { child_err: err }
    }
}

use std::{collections::HashMap, error::Error};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;

// Define the Task struct
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

// Define the TodoList struct
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

// Implement TodoList with a function to get todo from a file
impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        // Read the file content
        let mut file = fs::File::open(path).map_err(|e| Box::new(err::ReadErr::from(Box::new(e))) as Box<dyn Error>)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| Box::new(err::ReadErr::from(Box::new(e))) as Box<dyn Error>)?;

        // Deserialize the JSON content
        let todo_list: Result<TodoList, serde_json::Error> = serde_json::from_str(&contents);
        match todo_list {
            Ok(list) => {
                if list.tasks.is_empty() {
                    Err(Box::new(err::ParseErr::Empty) as Box<dyn Error>)
                } else {
                    Ok(list)
                }
            }
            Err(e) => Err(Box::new(err::ParseErr::Malformed(Box::new(e))) as Box<dyn Error>),
        }
    }
}
