use std::{env, fs, io, process};

use crate::path::list_executables_in_path;

type NewDir = Option<String>;
type StatusCode = i32;

pub enum Command {
    Exit(StatusCode),
    Echo(String),
    Type(Vec<String>),
    ChangeDir(NewDir),
    PrintWorkdir,
    Executable(String, Vec<String>),
}

#[derive(Debug)]
pub struct ParseCommandError(pub String);

pub struct ParseCommandResult {
    pub command_name: String,
    pub command: Result<Command, ParseCommandError>,
}

const BUILTIN_COMMANDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

pub fn parse_command(input: &str) -> ParseCommandResult {
    let mut segments = input.split_whitespace();
    let command_name = segments.next().expect("input cannot be empty");

    let command = match command_name {
        "exit" => segments
            .next()
            .map(|status_code_str| status_code_str.parse::<i32>())
            .unwrap_or(Ok(0))
            .map_err(|err| ParseCommandError(err.to_string()))
            .and_then(|status_code| {
                if segments.next() == None {
                    Ok(status_code)
                } else {
                    Err(ParseCommandError("too many arguments".to_string()))
                }
            })
            .map(|status_code| Command::Exit(status_code)),

        "echo" => {
            let message = segments.collect::<Vec<_>>().join(" ");

            Ok(Command::Echo(message))
        }

        "pwd" => {
            if let Some(_) = segments.next() {
                Err(ParseCommandError("too many arguments".to_string()))
            } else {
                Ok(Command::PrintWorkdir)
            }
        }

        "type" => Ok(Command::Type(
            segments.map(|str| str.to_owned()).collect::<Vec<_>>(),
        )),

        "cd" => {
            let newdir = segments.next().map(|newdir| newdir.to_string());
            if let Some(_) = segments.next() {
                Err(ParseCommandError("too many arguments".to_string()))
            } else {
                Ok(Command::ChangeDir(newdir))
            }
        }

        _ => {
            let executables_in_path = list_executables_in_path();
            if let Some(path) = executables_in_path.get(command_name) {
                let args = segments.map(|str| str.to_string()).collect::<Vec<_>>();
                Ok(Command::Executable(path.to_owned(), args))
            } else {
                Err(ParseCommandError("command not found".to_owned()))
            }
        }
    };

    return ParseCommandResult {
        command_name: command_name.to_string(),
        command,
    };
}

impl Command {
    pub fn execute(self: Self) -> Option<String> {
        match self {
            Command::Exit(status_code) => process::exit(status_code),
            Command::Echo(message) => Some(message),
            Command::Type(command_names) => {
                let executables_in_path = list_executables_in_path();

                Some(
                    command_names
                        .iter()
                        .map(|command_name| {
                            if BUILTIN_COMMANDS.contains(&command_name.as_str()) {
                                format!("{} is a shell builtin", command_name)
                            } else if let Some(path) = executables_in_path.get(command_name) {
                                format!("{} is {}", command_name, path)
                            } else {
                                format!("{}: not found", command_name)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            }

            Command::Executable(command_path, args) => {
                let _ = process::Command::new(command_path)
                    .args(args)
                    .spawn()
                    .ok()?
                    .wait_with_output();

                None
            }

            Command::PrintWorkdir => {
                let workdir = env::current_dir()
                    .map(|pathbuf| pathbuf.to_string_lossy().to_string())
                    .unwrap_or_else(|err| err.to_string());

                Some(workdir)
            }

            Command::ChangeDir(newdir) => {
                if newdir == None {
                    return None
                }

                let newdir = newdir.unwrap();
                let metadata = fs::metadata(&newdir);
                if let Err(err) = metadata {
                    let mut err_message = err.to_string();
                    if err_message.contains("No such file or directory") {
                        err_message = String::from(format!("cd: {}: No such file or directory", newdir))
                    }

                    return Some(err_message);
                } else if metadata.unwrap().is_dir() {
                    return match env::set_current_dir(newdir) {
                        Ok(_) => None,
                        Err(err) => Some(err.to_string())
                    } 
                } else {
                    return Some(format!("cd: {}, is not a directory", newdir));
                }
            }
        }
    }
}
