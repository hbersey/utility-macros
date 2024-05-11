//! A Rust library to emulate [Utility Types in TypeScript](https://www.typescriptlang.org/docs/handbook/utility-types.html)

pub use _um::{
    error::{Error, Result},
    partial::{HasPartial, Partial},
    readonly::{HasReadonly, Readonly},
    required::{HasRequired, Required},
    record::{HasRecord, Record},
};

#[doc(hidden)]
pub mod _um {
    pub use _um::*;
}

pub use _derive::*;
