#[allow(dead_code)]

type NewDir = String;
pub enum Command {
    ChangeDir(NewDir),
    Exit(),
}

#[derive(Debug)]
pub struct CommandParseError();

pub fn parse_command(input: &str) -> Result<Command, CommandParseError> {
    match input {
        "bye" => Ok(Command::Exit()),
        _ => Err(CommandParseError()),
    }
}
