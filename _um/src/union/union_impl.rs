use std::iter::Peekable;

use convert_case::{Case, Casing as _};
use proc_macro2::{Delimiter, Group, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use syn::Ident;

use crate::utils::{expect_token, peek_token, LiteralExt};

pub fn union_impl(item: TokenStream) -> TokenStream {
    let mut tokens = item.into_iter().peekable();
    let mut output = TokenStream::new();

    while let Some(_) = tokens.peek() {
        output.extend(impl_union(&mut tokens));
    }

    output
}

pub fn impl_union(tokens: &mut Peekable<impl Iterator<Item = TokenTree>>) -> TokenStream {
    let mut attrs = Vec::new();
    while let Some(_) = peek_token!(tokens, punct = '#') {
        tokens.next();
        attrs.push(expect_token!(tokens, group, delimiter = Delimiter::Bracket));
    }

    expect_token!(tokens, ident = "type");
    let ident = expect_token!(tokens, ident);
    expect_token!(tokens, punct = '=');

    let mut static_str = true;
    let mut strings = Vec::new();
    let mut types = Vec::new();

    loop {
        let t = tokens.next();
        match t {
            Some(TokenTree::Literal(literal)) if LiteralExt::is_str(&literal) => {
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
        return impl_static_strs(ident, attrs, strings);
    } else if strings.len() > 0 {
        panic!("expected either all variants to be &'static str or all variants to be types");
    }

    types_impls(ident, attrs, types)
}

fn impl_static_strs(ident: Ident, attrs: Vec<Group>, literals: Vec<Literal>) -> TokenStream {
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
        #(##attrs)*
        pub enum #ident {
            #(#variants),*
        }

        impl ::utility_macros::_um::union::StaticStrUnion for #ident {
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
                ::utility_macros::_um::union::StaticStrUnion::as_str(&self)
            }
        }

        impl ::std::str::FromStr for #ident {
            type Err = ::utility_macros::_um::error::Error;

            fn from_str(s: &str) -> ::utility_macros::_um::error::Result<Self> {
                ::utility_macros::_um::union::StaticStrUnion::try_from_str(s)
            }
        }
    }
}

fn types_impls(ident: Ident, attrs: Vec<Group>, types: Vec<Ident>) -> TokenStream {
    quote! {
        #(##attrs)*
        pub enum #ident {
            #(#types(#types)),*
        }

        impl ::utility_macros::_um::union::Union for #ident {
        }

        #(
            ::utility_macros::_um::_sa::assert_impl_all!(#types: Clone);

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
