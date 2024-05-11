use std::result::Result as StdResult;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Required field `{0}` is missing")]
    MissingField(&'static str),
    #[error("Duplicate key `{0}`")]
    DuplicateKey(&'static str),
    #[error("Missing key `{0}`")]
    MissingKey(&'static str),
}

pub type Result<T> = StdResult<T, Error>;
