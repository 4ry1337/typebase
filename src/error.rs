use thiserror::Error;

use crate::statement::StatementError;

pub type Result<T> = std::result::Result<T, SQLError>;

#[derive(Error, Debug)]
pub enum SQLError {
    #[error("Buffer error")]
    IOError(#[from] std::io::Error),
    #[error("StatementError: {0}")]
    StatementError(#[from] StatementError),
}
