use std::result::Result as StdResult;
use thiserror::Error as ThisError;

/// An error type for utility-macros
#[derive(ThisError, Debug)]
pub enum Error {
    /// A required field is None when calling `Partial::type_()` or `HasRequired::required()``
    #[error("Required field `{0}` is missing")]
    MissingField(&'static str),
    /// A duplicate key is found when calling `Record::from_entries()`
    #[error("Duplicate key `{0}`")]
    DuplicateKey(&'static str),
    /// A key is missing when calling `Record::from_entries()`
    #[error("Missing key `{0}`")]
    MissingKey(&'static str),
}

/// A type alias for `std::result::Result<T, Error>`
pub type Result<T> = StdResult<T, Error>;
