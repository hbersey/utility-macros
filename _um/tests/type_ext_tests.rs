use quote::{quote, ToTokens as _};
use syn::{parse_quote, PathArguments, Type};

use utility_macros_internals::utils::TypeExt as _;

#[test]
pub fn test_is_required() {
    let ty: Type = parse_quote!(i32);
    let opt: Type = parse_quote!(Option<i32>);
    assert!(ty.is_required());
    assert!(!opt.is_required());
}

#[test]
pub fn test_as_option() {
    let ty: Type = parse_quote!(i32);
    let ty_as_opt = ty.as_option().to_token_stream();
    let opt = quote!(Option<i32>);

    assert_eq!(ty_as_opt.to_string(), opt.to_string());

    let opt: Type = parse_quote!(Option<i32>);
    let opt2 = opt.clone();
    let opt_as_opt = opt.as_option().to_token_stream();
    assert_eq!(opt_as_opt.to_string(), opt2.to_token_stream().to_string());
}

#[test]
pub fn test_as_required() {
    let ty: Type = parse_quote!(i32);
    let ty2 = ty.clone();
    let ty_as_req = ty.as_required().to_token_stream();
    assert_eq!(ty_as_req.to_string(), ty2.to_token_stream().to_string());

    let opt: Type = parse_quote!(Option<i32>);
    let req = opt.as_required().to_token_stream();
    let req2: Type = parse_quote!(i32);
    assert_eq!(req.to_string(), req2.to_token_stream().to_string());

}

