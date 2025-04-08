use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap_or_else(|err| {
            eprintln!("Error opening or creating file: {}", err);
            panic!("File operation failed");
        });

    if let Err(err) = writeln!(file, "{}", content) {
        eprintln!("Error writing to file: {}", err);
        panic!("File write failed");
    }
}
