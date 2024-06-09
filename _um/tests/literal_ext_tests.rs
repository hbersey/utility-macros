use proc_macro2::Literal;
use utility_macros_internals::utils::LiteralExt as _;

#[test]
pub fn test_is_str() {
    let s = Literal::string("hello");
    let n = Literal::u32_unsuffixed(5);

    assert!(s.is_str());
    assert!(!n.is_str());
}

#[test]
pub fn test_as_string() {
    let s = Literal::string("hello");
    let n = Literal::u32_unsuffixed(5);

    assert_eq!(s.as_string().unwrap(), "hello");
    assert!(n.as_string().is_none());
}
