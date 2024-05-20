use proc_macro2::Literal;
use quote::{quote, ToTokens};
use utility_macros_internals::union::union_impl::*;

#[test]
fn test_is_static_str() {
    assert!(is_static_str(&Literal::string("Hello, World!")));
    assert!(!is_static_str(&Literal::u8_unsuffixed(12)));
}

