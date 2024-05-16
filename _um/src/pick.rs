use crate::error::Result;

/// A trait for types that have a pick representation.
pub trait HasPick {
    /// The pick representation of the type.
    type Pick;

    /// Converts the type to its pick representation.
    fn pick(&self) -> Result<Self::Pick>;
}

/// A trait for pick representations of types.
pub trait Pick {
    /// The type that the pick representation is for.
    type Type;
}
