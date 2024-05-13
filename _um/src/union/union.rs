use static_assertions::assert_obj_safe;

assert_obj_safe!(Union);

/// A trait for representing a union
/// Don't implement this trait manually, use the `union` macro instead
pub trait Union {}
