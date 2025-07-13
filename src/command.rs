use thiserror::Error;

pub struct Command {
    r#type: CommandType,
}

pub enum CommandType {
    Exit,
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Unrecognised command: `{0}`")]
    Unrecognised(String),
}

impl Command {
    pub fn new(command: &str) -> Result<Self, CommandError> {
        if command == ".exit" {
            return Ok(Self {
                r#type: CommandType::Exit,
            });
        }
        return Err(CommandError::Unrecognised(command.into()));
    }

    pub fn execute(&self) {
        match self.r#type {
            CommandType::Exit => {
                std::process::exit(0);
            }
        }
    }
}
