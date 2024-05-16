//! A Rust library to emulate [Utility Types in TypeScript](https://www.typescriptlang.org/docs/handbook/utility-types.html)

pub use _um::{
    error::{Error, Result},
    partial::{HasPartial, Partial},
    readonly::{HasReadonly, Readonly},
    record::{HasRecord, Record},
    required::{HasRequired, Required},
    pick::{HasPick, Pick},
    union::{static_str_union::StaticStrUnion, union::Union},
};

#[doc(hidden)]
pub mod _um {
    pub use _um::*;
}

pub use _derive::*;
