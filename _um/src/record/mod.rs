use crate::error::Result;
use std::ops::{Index, IndexMut};

mod record_impl;
pub use record_impl::record_impl as derive;

/// A trait for types that have a record representation.
pub trait HasRecord: Sized {
    /// The record representation of the type.
    type Record: Record<Keys = Self>;

    /// Converts the type to its record representation.
    fn as_str(&self) -> &'static str;
    /// Converts the type to its record representation.
    fn try_from_str(s: &str) -> Result<Self>;
}

/// A trait for record representations of types.
pub trait Record: Index<Self::Keys> + IndexMut<Self::Keys> + Sized {
    /// The type that the record representation is for.
    type Keys: HasRecord<Record = Self>;
    /// The type that of the records values.
    type Type;

    /// The number of fields in the record.
    const N: usize;

    /// The keys of the record.
    fn keys() -> Vec<Self::Keys>;

    /// The values of the record.
    fn values(&self) -> Vec<&Self::Type>;
    /// The mutable values of the record.
    fn values_mut(&mut self) -> Vec<&mut Self::Type>;

    /// The entries of the record.
    fn entries(&self) -> Vec<(Self::Keys, &Self::Type)>;
    /// The mutable entries of the record.
    /// Note. that the keys are not mutable.
    fn entires_mut(&mut self) -> Vec<(Self::Keys, &mut Self::Type)>;

    /// Try convert a vector of entries to the record representation.
    fn try_from_entries(entries: Vec<(Self::Keys, Self::Type)>) -> Result<Self>;
}
