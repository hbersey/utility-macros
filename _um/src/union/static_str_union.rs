use static_assertions::assert_obj_safe;

use crate::{error::Result, union::union::Union};

assert_obj_safe!(StaticStrUnion);

/// A trait for representing a `&'static str` union
/// Don't implement this trait manually, use the `union` macro instead
pub trait StaticStrUnion: Union {
    /// Return all variants as `&'static str`
    fn strs() -> Vec<&'static str>
    where
        Self: Sized;

    /// Returns the `&'static str` representation of the variant
    fn as_str(&self) -> &'static str;

    /// Tries to convert a `&'static str` to `Self`
    fn try_from_str(value: &str) -> Result<Self>
    where
        Self: Sized;
}

impl<T> Union for T where T: StaticStrUnion {}
