#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{self, Write};
use commands::parse_command;

mod commands;

fn eval(input: &str) -> String {
    let trimmed_input = input.trim_end_matches("\n");

    let command = parse_command(trimmed_input);
    if let Err(_) = command {
        return format!("{}: command not found", &trimmed_input)
    }

    unreachable!();
}

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        stdin.read_line(&mut input).unwrap();
        let output = eval(&input);
        println!("{}", output);
    }
}
