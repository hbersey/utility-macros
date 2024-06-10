use utility_macros_internals::attributes::{
    parse_container_attribute, parse_container_attributes, ContainerAttribute,
    ContainerAttributes as _,
};

use convert_case::Case;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Attribute, Ident};

#[test]
fn test_parse_container_attribute() {
    let tokens = quote! {
        name = "Foo", case_all = "snake", derive = "Debug", where = "T: Debug", pizza = "yes please!"
    };
    let mut tokens = tokens.into_iter().peekable();

    let attr = parse_container_attribute(&mut tokens).unwrap();
    assert_eq!(attr, ContainerAttribute::Name("Foo".to_string()));
    tokens.next();

    let attr = parse_container_attribute(&mut tokens).unwrap();
    assert_eq!(attr, ContainerAttribute::CaseAll(Case::Snake));
    tokens.next();

    let attr = parse_container_attribute(&mut tokens).unwrap();
    assert_eq!(attr, ContainerAttribute::Derive("Debug".to_string()));
    tokens.next();

    let attr = parse_container_attribute(&mut tokens).unwrap();
    assert_eq!(attr, ContainerAttribute::Where("T: Debug".to_string()));
    tokens.next();

    let attr = parse_container_attribute(&mut tokens);
    assert!(attr.is_err());
}

#[test]
fn test_ident() {
    let ident = Ident::new("Foo", Span::call_site());

    let attrs1 = Vec::<ContainerAttribute>::new();
    let attrs2 = vec![ContainerAttribute::Name("Bar".to_string())];

    assert_eq!(attrs1.ident(ident.clone()), "Foo");
    assert_eq!(attrs2.ident(ident.clone()), "Bar");
}

#[test]
fn test_field_ident() {
    let ident = Ident::new("foo", Span::call_site());

    let attrs1 = Vec::<ContainerAttribute>::new();
    let attrs2 = vec![ContainerAttribute::CaseAll(Case::Upper)];

    assert_eq!(attrs1.field_ident(ident.clone()), "foo");
    assert_eq!(attrs2.field_ident(ident.clone()), "FOO");
}

#[test]
fn test_derive_statement() {
    let attrs1 = Vec::<ContainerAttribute>::new();
    let attrs2 = vec![ContainerAttribute::Derive("Debug".to_string())];
    let attrs_err = vec![ContainerAttribute::Derive("A\"B".to_string())];

    assert_eq!(
        attrs1.derive_statement("A, B, C").unwrap().to_string(),
        quote! {#[derive(A, B, C)]}.to_string()
    );
    assert_eq!(
        attrs2.derive_statement("A, B, C").unwrap().to_string(),
        quote! { #[derive(Debug)] }.to_string()
    );
    assert!(attrs_err.derive_statement("A, B, C").is_err());
}

#[test]
fn test_where_statement() {
    let attrs1 = Vec::<ContainerAttribute>::new();
    let attrs2 = vec![ContainerAttribute::Where("T: Debug".to_string())];
    let attrs_err = vec![ContainerAttribute::Where("A\"B".to_string())];

    assert_eq!(
        attrs1.where_statement().unwrap().to_string(),
        quote! {}.to_string()
    );
    assert_eq!(
        attrs2.where_statement().unwrap().to_string(),
        quote! { where T: Debug }.to_string()
    );
    assert!(attrs_err.where_statement().is_err());
}

#[test]
fn test_no_name() {
    let attrs = vec![ContainerAttribute::CaseAll(Case::Upper)];
    assert_eq!(attrs.name(), None);
}

#[test]
fn test_no_case_all() {
    let attrs = vec![ContainerAttribute::Name("Foo".to_string())];
    assert_eq!(attrs.case_all(), None);
}

#[test]
fn test_no_derive() {
    let attrs = vec![ContainerAttribute::Name("Foo".to_string())];
    assert_eq!(attrs.derive(), None);
}

#[test]
fn test_no_where_clause() {
    let attrs = vec![ContainerAttribute::Name("Foo".to_string())];
    assert_eq!(attrs.where_clause(), None);
}

#[test]
fn test_parse_container_attributes() {
    let attrs: Vec<Attribute> = vec![
        parse_quote! {
            #[utility_macros(name = "Foo", case_all = "snake", derive = "Debug", where = "T: Debug")]
        },
        parse_quote! {
            #[foo(bar)]
        },
        parse_quote! {
            #[baz]
        },
    ];

    let attrs = parse_container_attributes(attrs);
    assert!(attrs.is_ok());

    let attrs_err = vec![parse_quote! { #[utility_macros(foo = "bar")] }];
    let attrs_err = parse_container_attributes(attrs_err);
    assert!(attrs_err.is_err());
}
