pub mod error;

pub mod partial;
pub mod required;
pub mod readonly;
pub mod record;

pub mod container_attributes;
pub mod field_attributes;

pub mod option;

pub mod _sa {
    pub use static_assertions::assert_impl_all;
}
