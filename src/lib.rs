//! A Rust library to emulate [Utility Types in TypeScript](https://www.typescriptlang.org/docs/handbook/utility-types.html)

pub use _um::{
    error::{Error, Result},
    partial::{HasPartial, Partial},
    pick::{HasPick, Pick},
    readonly::{HasReadonly, Readonly},
    record::{HasRecord, Record},
    required::{HasRequired, Required},
    union::{StaticStrUnion, Union},
};

#[doc(hidden)]
pub mod _um {
    pub use _um::*;
}

pub use _derive::*;
