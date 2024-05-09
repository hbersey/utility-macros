pub mod error;

pub mod partial;
pub mod required;

pub mod container_attributes;
pub mod field_attributes;

#[macro_export]
macro_rules! path_ {
    ($root:ident$(::$e:ident)*) => {
        ::utility_macros::_utility_macros::$root$(::$e)*
    };
}

pub mod option;

pub mod _sa {
    pub use static_assertions::assert_impl_all;
}
