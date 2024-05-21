use std::{iter::Peekable, str::FromStr};

use convert_case::{Case, Casing as _};
use proc_macro2::{TokenStream, TokenTree};
use syn::{Attribute, Ident, Meta};

use crate::{
    error::{Error, Result},
    expect_token::expect_token,
    utils::CaseExt as _,
};

pub enum ContainerAttribute {
    Name(String),
    CaseAll(Case),
    Derive(String),
    Where(String),
}

fn parse_container_attribute(
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<ContainerAttribute> {
    let ident = expect_token!(tokens, ident);

    match ident.to_string().as_str() {
        // #[utility_macros(name="Name")]
        "name" => {
            expect_token!(tokens, punct = '=');
            let name = expect_token!(tokens, string);
            Ok(ContainerAttribute::Name(name))
        }
        // #[utility_macros(case_all="UPPER_SNAKE")]
        "case_all" => {
            expect_token!(tokens, punct = '=');
            let case = expect_token!(tokens, string);
            Ok(ContainerAttribute::CaseAll(
                Case::from_str(&case.to_string()).unwrap(),
            ))
        }
        // #[utility_macros(derive="Debug, Clone")]
        "derive" => {
            expect_token!(tokens, punct = '=');
            let derive = expect_token!(tokens, string);
            Ok(ContainerAttribute::Derive(derive))
        }
        // #[utility_macros(where="T: Debug + Clone")]
        "where" => {
            expect_token!(tokens, punct = '=');
            let where_clause = expect_token!(tokens, string);
            Ok(ContainerAttribute::Where(where_clause))
        }
        s => Err(Error::InvalidAttributeItem(s.to_string())),
    }
}

pub trait ContainerAttributes {
    fn name(&self) -> Option<&str>;
    fn ident(&self, default: Ident) -> Ident {
        self.name()
            .map(|name| Ident::new(name, default.span()))
            .unwrap_or(default)
    }

    fn case_all(&self) -> Option<Case>;
    fn field_ident(&self, ident: Ident) -> Ident {
        match self.case_all() {
            Some(case) => Ident::new(ident.to_string().to_case(case).as_str(), ident.span()),
            None => ident.clone(),
        }
    }

    fn derive(&self) -> Option<&str>;
    fn derive_statement(&self, default: &str) -> Result<TokenStream> {
        let inner = self.derive().unwrap_or(default);
        let inner = TokenStream::from_str(inner)
            .map_err(|_| Error::InvalidDeriveStatement(inner.to_string()))?;
        Ok(quote::quote! {
            #[derive(#inner)]
        })
    }

    fn where_clause(&self) -> Option<&str>;
    fn where_statement(&self) -> TokenStream {
        match self.where_clause() {
            Some(where_clause) => {
                quote::quote! {
                    where #where_clause
                }
            }
            None => quote::quote! {},
        }
    }
}

impl ContainerAttributes for Vec<ContainerAttribute> {
    fn name(&self) -> Option<&str> {
        self.iter()
            .filter_map(|attr| match attr {
                ContainerAttribute::Name(name) => Some(name.as_str()),
                _ => None,
            })
            .next()
    }

    fn case_all(&self) -> Option<Case> {
        self.iter()
            .filter_map(|attr| match attr {
                ContainerAttribute::CaseAll(case) => Some(*case),
                _ => None,
            })
            .next()
    }

    fn derive(&self) -> Option<&str> {
        self.iter()
            .filter_map(|attr| match attr {
                ContainerAttribute::Derive(derive) => Some(derive.as_str()),
                _ => None,
            })
            .next()
    }

    fn where_clause(&self) -> Option<&str> {
        self.iter()
            .filter_map(|attr| match attr {
                ContainerAttribute::Where(where_clause) => Some(where_clause.as_str()),
                _ => None,
            })
            .next()
    }
}

pub fn parse_container_attributes(type_attrs: Vec<Attribute>) -> Result<Vec<ContainerAttribute>> {
    let mut container_attrs = Vec::new();

    for attr in type_attrs {
        let Meta::List(meta) = attr.meta else {
            continue;
        };

        if !meta.path.is_ident("utility_macros") {
            continue;
        }

        let mut tokens = meta.tokens.into_iter().peekable();
        let attr = parse_container_attribute(&mut tokens)?;
        container_attrs.push(attr);
    }

    Ok(container_attrs)
}
