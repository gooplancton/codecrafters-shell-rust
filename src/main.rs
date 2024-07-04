#![allow(dead_code)]
#![allow(unused_imports)]

use commands::parse_command;
use std::io::{self, Write};

mod commands;

fn eval(input: &str) -> String {
    let trimmed_input = input.trim_end_matches("\n");

    let command = parse_command(trimmed_input);
    if let Err(_) = command {
        return format!("{}: command not found", &trimmed_input);
    }

    unreachable!();
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let output = eval(&input);
        println!("{}", output);
    }
}
