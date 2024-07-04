#[allow(unused_imports)]
use std::io::{self, Write};

use commands::parse_command;

mod commands;

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input = input.trim_end_matches("\n").to_owned();

    let command = parse_command(&input);
    if let Err(_) = command {
        print!("{}: command not found", &input)
    }
}
