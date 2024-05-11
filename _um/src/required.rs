use crate::error::Result;

/// A trait for types that have a required representation.
pub trait HasRequired {
    /// The required representation of the type.
    type Required;

    /// Converts the type to its required representation.
    fn required(&self) -> Result<Self::Required>;
}

/// A trait for required representations of types.
pub trait Required {
    /// The type that the required representation is for.
    type Type;

    /// Converts the required representation to the original type.
    fn type_(&self) -> Self::Type;
}
