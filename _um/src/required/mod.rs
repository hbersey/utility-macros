use crate::error::Result;

mod required_impl;
pub use required_impl::required_impl as derive;

/// A trait for types that have a required representation.
pub trait HasRequired {
    /// The required representation of the type.
    type Required: Required<Type = Self>;

    /// Converts the type to its required representation.
    fn required(&self) -> Result<Self::Required>;
}

/// A trait for required representations of types.
pub trait Required {
    /// The type that the required representation is for.
    type Type: HasRequired<Required = Self>;

    /// Converts the required representation to the original type.
    fn type_(&self) -> Self::Type;
}
