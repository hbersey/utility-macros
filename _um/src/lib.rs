pub mod error;

pub mod partial;
pub mod readonly;
pub mod record;
pub mod required;

pub mod case;
pub mod option;

pub mod derive;

pub mod _sa {
    pub use static_assertions::assert_impl_all;
}
