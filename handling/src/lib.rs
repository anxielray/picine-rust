use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Failed to open or create file");

    if content.ends_with('\n') {
        write!(file, "{}", content).expect("Failed to write to file");
    } else {
        write!(file, "{}\n", content).expect("Failed to write to file");
    }
}
