pub mod error;

pub mod partial;
pub mod pick;
pub mod readonly;
pub mod record;
pub mod required;

pub mod union;

pub mod attributes;

pub mod utils;

pub mod _sa {
    pub use static_assertions::assert_impl_all;
}
