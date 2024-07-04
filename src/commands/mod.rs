use std::process;

type NewDir = String;
type StatusCode = i32;

pub enum Command {
    Exit(StatusCode),
    Echo(String),
    ChangeDir(NewDir),
}

#[derive(Debug)]
pub struct ParseCommandError(pub String);

pub struct ParseCommandResult {
    pub command_name: String,
    pub command: Result<Command, ParseCommandError>,
}

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

        _ => Err(ParseCommandError("command not found".to_owned())),
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
            Command::ChangeDir(_) => todo!(),
        }
    }
}
