//! A Rust library to emulate [Utility Types in TypeScript](https://www.typescriptlang.org/docs/handbook/utility-types.html)

pub use _um::{
    error::{Error, Result},
    partial::{HasPartial, Partial},
    readonly::{HasReadonly, Readonly},
    record::{HasRecord, Record},
    required::{HasRequired, Required},
    string_union::StringUnion,
};

#[doc(hidden)]
pub mod _um {
    pub use _um::*;
}

pub use _derive::*;
