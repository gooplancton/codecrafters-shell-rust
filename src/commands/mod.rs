use std::process;

type NewDir = String;
type StatusCode = i32;

pub enum Command {
    Exit(StatusCode),
    Echo(String),
    ChangeDir(NewDir),
}

#[derive(Debug)]
pub struct CommandParseError(pub String);

pub fn parse_command(input: &str) -> Result<Command, CommandParseError> {
    let segments: Vec<&str> = input.split_whitespace().collect();

    match &segments[..] {
        &["exit"] => Ok(Command::Exit(0)),
        &["exit", status] => Ok(Command::Exit(
            status
                .parse::<i32>()
                .map_err(|err| CommandParseError(err.to_string()))?,
        )),
        // &["exit", _, _] => Err(CommandParseError("too many arguments".to_string())),
        // &["echo", rest @ ..] => Ok(Command::Echo(rest.join(" "))),
        _ => Err(CommandParseError("command not found".to_owned())),
    }
}

impl Command {
    pub fn execute(self: &Self) -> Option<String> {
        match self {
            Command::Exit(status_code) => process::exit(*status_code),
            Command::Echo(_) => todo!(),
            Command::ChangeDir(_) => todo!(),
        }
    }
}
