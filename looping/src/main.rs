use std::io::{self, Write};

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. \
                  I am essential to creation, and I surround every place. What am I?";
    let correct_answer = "The letter e";
    let mut correct = false;
    let mut attempts = 0;

    while !correct {
        attempts += 1;
        println!("\n[RIDDLE]: {}", riddle);
        print!("Your answer: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer = answer.trim().to_lowercase();

        if answer == correct_answer.to_lowercase() {
            println!("Correct! It only took you {} attempts. Great!! You are awesome!", attempts);
            correct = true;
        } else {
            println!("Incorrect. Try again.");
        }
    }
}
