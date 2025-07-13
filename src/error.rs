use thiserror::Error;

use crate::{command::CommandError, statement::StatementError};

pub type Result<T> = std::result::Result<T, SQLError>;

#[derive(Error, Debug)]
pub enum SQLError {
    #[error("Buffer error")]
    IOError(#[from] std::io::Error),
    #[error("StatementError: {0}")]
    StatementError(#[from] StatementError),
    #[error("CommandError: {0}")]
    CommandError(#[from] CommandError),
}
