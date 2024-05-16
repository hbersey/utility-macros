use crate::error::Result;

/// A trait for types that have a partial representation.
pub trait HasPartial {
    /// The partial representation of the type.
    type Partial: Partial<Type = Self>;

    /// Converts the type to its partial representation.
    fn partial(&self) -> Self::Partial;
}

/// A trait for partial representations of types.
pub trait Partial {
    /// The type that the partial representation is for.
    type Type: HasPartial<Partial = Self>;

    /// Converts the partial representation to the original type. 
    fn type_(&self) -> Result<Self::Type>;
}
