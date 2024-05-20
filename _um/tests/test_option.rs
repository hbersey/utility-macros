use quote::ToTokens;
use syn::{parse_quote, Type};
use utility_macros_internals::option::*;

#[test]
fn test_is_option() {
    assert!(is_option(&parse_quote!(Option<i32>)));
    assert!(!is_option(&parse_quote!(i32)));
}

#[test]
fn test_as_option() {
    let ty: Type = parse_quote!(i32);
    let opt: Type = parse_quote!(Option<i32>);

    assert_eq!(
        as_option(&ty).to_token_stream().to_string(),
        opt.to_token_stream().to_string()
    );

    assert_eq!(
        as_option(&opt).to_token_stream().to_string(),
        opt.to_token_stream().to_string()
    );
}

#[test]
fn test_as_required() {
    let ty: Type = parse_quote!(i32);
    let opt: Type = parse_quote!(Option<i32>);

    assert_eq!(
        as_required(&ty).to_token_stream().to_string(),
        ty.to_token_stream().to_string()
    );

    assert_eq!(
        as_required(&opt).to_token_stream().to_string(),
        ty.to_token_stream().to_string()
    );
}
