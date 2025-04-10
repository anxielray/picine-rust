pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    // Associated function to create a new Flag
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Flag {
            short_hand: format!("-{}", name),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, std::num::ParseFloatError>;

use std::collections::HashMap;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    // Method to add a flag and its associated function
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert((flag.short_hand.clone(), flag.long_hand.clone()), func);
    }

    // Method to execute the function associated with a flag
    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some(func) = self.flags.get(&(input.to_string(), input.to_string())) {
            if argv.len() == 2 {
                func(argv[0], argv[1]).map_err(|e| e.to_string())
            } else {
                Err("Invalid number of arguments".to_string())
            }
        } else {
            Err("Flag not recognized".to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, std::num::ParseFloatError> {
    let num_a: f64 = a.parse()?;
    let num_b: f64 = b.parse()?;
    Ok((num_a / num_b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, std::num::ParseFloatError> {
    let num_a: f64 = a.parse()?;
    let num_b: f64 = b.parse()?;
    Ok((num_a % num_b).to_string())
}
