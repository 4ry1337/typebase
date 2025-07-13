use thiserror::Error;

pub struct Statement {
    r#type: StatementType,
}

pub enum StatementType {
    StatementInsert,
    StatementSelect,
}

#[derive(Error, Debug)]
pub enum StatementError {
    #[error("Unrecognised keyword at start of `{0}`")]
    Unrecognised(String),
}

impl Statement {
    pub fn new(statement: &str) -> Result<Self, StatementError> {
        if statement.starts_with("insert") {
            return Ok(Self {
                r#type: StatementType::StatementInsert,
            });
        }
        if statement.starts_with("select") {
            return Ok(Self {
                r#type: StatementType::StatementSelect,
            });
        }
        return Err(StatementError::Unrecognised(statement.into()));
    }

    pub fn execute(&self) {
        match self.r#type {
            StatementType::StatementInsert => {
                println!("This is where we would do an insert.");
            }
            StatementType::StatementSelect => {
                println!("This is where we would do a select.");
            }
        }
    }
}
