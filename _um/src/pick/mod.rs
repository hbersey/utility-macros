use crate::error::Result;

mod pick_impl;
pub use pick_impl::pick_impl as derive;

/// A trait for types that have a pick representation.
pub trait HasPick {
    /// The pick representation of the type.
    type Pick: Pick<Type = Self>;

    /// Converts the type to its pick representation.
    fn pick(&self) -> Result<Self::Pick>;
}

/// A trait for pick representations of types.
pub trait Pick {
    /// The type that the pick representation is for.
    type Type: HasPick<Pick = Self>;
}
