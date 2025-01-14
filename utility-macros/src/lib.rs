pub use derive::*;

mod error;
pub use error::{Error, Result};

mod partial;
pub use partial::{HasPartial, Partial};
