use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter a string >> ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Error reading from STDIN");
    print!("You input: {}", input);
}
