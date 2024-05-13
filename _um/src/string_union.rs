use core::panic;

use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use syn::Ident;

use crate::error::Result;

pub trait StringUnion: Sized {
    /// Returns all variants as Vec<&'static str>
    fn strings() -> Vec<&'static str>;
    /// Returns all variants as Vec<Self>
    fn variants() -> Vec<Self>;

    /// Returns the string representation of the variant
    fn as_str(&self) -> &'static str;

    /// Tries to convert a string to a Self
    fn try_from_str(value: &str) -> Result<Self>;
}

pub fn string_union_impl(item: TokenStream) -> TokenStream {
    let mut tokens = item.into_iter();

    match tokens.next() {
        Some(TokenTree::Ident(ident)) if ident == "type" => {}
        _ => panic!("expected `type`"),
    }

    let ident = match tokens.next() {
        Some(TokenTree::Ident(ident)) => ident,
        _ => panic!("expected identifier"),
    };

    let _ = match tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {}
        _ => panic!("expected `=`"),
    };

    let mut strings = Vec::new();
    let mut variants = Vec::new();

    loop {
        let Some(TokenTree::Literal(literal)) = tokens.next() else {
            panic!("expected string literal");
        };

        strings.push(literal.clone());

        let variant = {
            let s = literal.to_string();
            let s = s.trim_matches('"');
            let s = s.to_case(Case::Pascal).clone();
            let s = s.replace(" ", "");
            s
        };

        variants.push(Ident::new(&variant, Span::call_site()));

        match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '|' => {}
            Some(TokenTree::Punct(punct)) if punct.as_char() == ';' => break,
            _ => panic!("expected `|` or `;`"),
        }
    }

    quote! {
        #[derive(Debug, PartialEq, Clone)]
        pub enum #ident {
            #(#variants),*
        }

        impl ::utility_macros::StringUnion for #ident {
            fn strings() -> Vec<&'static str> {
                vec![#(#strings),*]
            }

            fn variants() -> Vec<Self> {
                vec![#(#ident::#variants),*]
            }

            fn as_str(&self) -> &'static str {
                match self {
                    #(Self::#variants => #strings),*
                }
            }

            fn try_from_str(value: &str) -> ::utility_macros::_um::error::Result<Self> {
                match value {
                    #(#strings => {Ok(Self::#variants)}),*
                    _ => {
                        Err(::utility_macros::_um::error::Error::InvalidVariant(value.to_string()))
                    },
                }
            }
        }

        impl ::core::convert::Into<&'static str> for #ident {
            fn into(self) -> &'static str {
                ::utility_macros::StringUnion::as_str(&self)
            }
        }

        impl ::core::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                write!(f, "{}", ::utility_macros::StringUnion::as_str(self))
            }
        }

        impl ::core::str::FromStr for #ident {
            type Err = ::utility_macros::_um::error::Error;

            fn from_str(value: &str) -> ::utility_macros::_um::error::Result<Self> {
                <Self as ::utility_macros::StringUnion>::try_from_str(value)
            }
        }

        impl ::core::convert::TryFrom<String> for #ident {
            type Error = ::utility_macros::_um::error::Error;

            fn try_from(value: String) -> ::utility_macros::_um::error::Result<Self> {
                <Self as ::utility_macros::StringUnion>::try_from_str(&value)
            }
        }

        impl PartialEq<&str> for #ident {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl PartialEq<String> for #ident {
            fn eq(&self, other: &String) -> bool {
                self.as_str() == other.as_str()
            }
        }
    }
}
