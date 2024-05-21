pub mod error;

pub mod partial;
pub mod pick;
pub mod readonly;
pub mod record;
pub mod required;

pub mod union;

#[deprecated(note = "Use `_um::utils::CaseExt` instead.")]
pub mod case;
pub mod expect_token;
#[deprecated(note = "Use `_um::utils::OptionExt` instead.")]
pub mod option;

pub mod attributes;
pub mod derive;

pub mod utils;

pub mod _sa {
    pub use static_assertions::assert_impl_all;
}
