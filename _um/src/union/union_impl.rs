use convert_case::{Case, Casing as _};
use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::quote;
use syn::Ident;

use crate::expect_token::expect_token;

fn is_static_str(literal: &Literal) -> bool {
    let s = literal.to_string();
    s.starts_with("\"") && s.ends_with("\"")
}

pub fn union_impl(item: TokenStream) -> TokenStream {
    let mut tokens = item.into_iter();

    expect_token!(tokens, ident = "type");
    let ident = expect_token!(tokens, ident);
    expect_token!(tokens, punct = '=');

    let mut static_str = true;
    let mut strings = Vec::new();
    let mut types = Vec::new();

    loop {
        let t = tokens.next();
        match t {
            Some(TokenTree::Literal(literal)) if is_static_str(&literal) => {
                strings.push(literal);
            }
            Some(TokenTree::Ident(ident)) => {
                static_str = false;
                types.push(ident);
            }
            _ => {
                panic!("expected &'static str literal or identifier")
            }
        }

        match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '|' => {}
            Some(TokenTree::Punct(punct)) if punct.as_char() == ';' => break,
            _ => panic!("expected `|` or `;`"),
        }
    }

    if static_str {
        return static_strs_impl(ident, strings);
    }

    if strings.len() > 0 {
        panic!("expected either all variants to be &'static str or all variants to be types");
    }

    types_impls(ident, types)
}

fn static_strs_impl(ident: Ident, literals: Vec<Literal>) -> TokenStream {
    let variants = literals
        .clone()
        .into_iter()
        .map(|literal| {
            let s = literal.to_string();
            let s = s.trim_matches('"');
            let s = s.to_case(Case::Pascal).clone();
            let s = s.replace(" ", "");
            Ident::new(&s, Span::call_site())
        })
        .collect::<Vec<_>>();

    quote! {
        pub enum #ident {
            #(#variants),*
        }

        impl ::utility_macros::_um::union::static_str_union::StaticStrUnion for #ident {
            fn strs() -> Vec<&'static str>
            where
                Self: Sized
            {
                vec![#(#literals),*]
            }

            fn as_str(&self) -> &'static str {
                match self {
                    #(Self::#variants => #literals),*
                }
            }

            fn try_from_str(value: &str) -> ::utility_macros::_um::error::Result<Self>
            where
                Self: Sized
            {
                match value {
                    #(#literals => Ok(Self::#variants),)*
                    _ => Err(::utility_macros::_um::error::Error::InvalidVariant(value.to_string()))
                }
            }
        }

        impl Into<&'static str> for #ident {
            fn into(self) -> &'static str {
                ::utility_macros::_um::union::static_str_union::StaticStrUnion::as_str(&self)
            }
        }

        impl ::std::str::FromStr for #ident {
            type Err = ::utility_macros::_um::error::Error;

            fn from_str(s: &str) -> ::utility_macros::_um::error::Result<Self> {
                ::utility_macros::_um::union::static_str_union::StaticStrUnion::try_from_str(s)
            }
        }
    }
}

fn types_impls(ident: Ident, types: Vec<Ident>) -> TokenStream {
    quote! {
        pub enum #ident {
            #(#types(#types)),*
        }

        impl ::utility_macros::_um::union::union::Union for #ident {
        }

        #(
            impl TryInto<#types> for #ident {
                type Error = ::utility_macros::_um::error::Error;

                fn try_into(self) -> ::utility_macros::_um::error::Result<#types> {
                    match self {
                        Self::#types(value) => Ok(value),
                        _ => Err(::utility_macros::_um::error::Error::InvalidVariant(stringify!(#types).to_string()))
                    }
                }
            }

            impl From<#types> for #ident {
                fn from(value: #types) -> Self {
                    Self::#types(value)
                }
            }
        )*
    }
}
