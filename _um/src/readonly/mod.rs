mod readonly_impl;
pub use readonly_impl::readonly_impl as derive;

/// A trait for types that have a readonly version.
pub trait HasReadonly {
    /// The readonly version of the type.
    type Readonly: Readonly<Type = Self>;

    /// Converts the type to its readonly version.
    fn readonly(&self) -> Self::Readonly;
}

/// A trait for readonly versions of types.
pub trait Readonly {
    /// The type that the readonly version is for.
    type Type: HasReadonly<Readonly = Self>;

    /// Converts the readonly version to the original type.
    fn type_(&self) -> Self::Type;
}
