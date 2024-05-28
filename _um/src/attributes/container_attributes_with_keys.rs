use std::iter::Peekable;

use convert_case::{Case, Casing as _};
use proc_macro2::{Span, TokenTree};
use syn::{Attribute, Ident, Meta};

use crate::utils::{expect_token, peek_token, ResultExt as _};

use super::{parse_container_attribute, ContainerAttribute, ContainerAttributes};

pub struct ContainerAttributesWithKeys {
    name: String,
    keys: Vec<String>,
    attributes: Vec<ContainerAttribute>,
}

impl ContainerAttributesWithKeys {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn ident(&self) -> Ident {
        Ident::new(self.name.as_str(), Span::call_site())
    }

    pub fn keys(&self) -> &Vec<String> {
        &self.keys
    }

    pub fn key_idents(&self) -> Vec<Ident> {
        self.keys
            .iter()
            .map(|key| Ident::new(key, Span::call_site()))
            .collect()
    }

    pub fn case_all(&self) -> Option<Case> {
        self.attributes.case_all()
    }

    pub fn field_variant_ident(&self, ident: Ident) -> Ident {
        match self.case_all() {
            Some(case) => Ident::new(ident.to_string().to_case(case).as_str(), ident.span()),
            None => ident,
        }
    }

    pub fn derive(&self) -> Option<&str> {
        self.attributes.derive()
    }

    pub fn where_clause(&self) -> Option<&str> {
        self.attributes.derive()
    }
}

fn parse_container_attributes_with_keys_tokens(
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> ContainerAttributesWithKeys {
    let name = expect_token!(tokens, string);
    expect_token!(tokens, =>);

    let mut keys = Vec::new();
    loop {
        let key = expect_token!(tokens, string);
        keys.push(key);
        if tokens.peek().is_none() {
            break;
        } else if peek_token!(tokens, punct = ',').is_some() {
            tokens.next();
            break;
        }

        expect_token!(tokens, punct = '|');
    }

    let mut attributes = Vec::new();
    while tokens.peek().is_some() {
        let attribute = parse_container_attribute(tokens).or_panic();
        attributes.push(attribute);
    }

    ContainerAttributesWithKeys {
        name,
        keys,
        attributes,
    }
}

pub fn parse_container_attributes_with_keys(
    attributes: Vec<Attribute>,
) -> Vec<ContainerAttributesWithKeys> {
    let mut container_attributes = Vec::new();

    for attr in attributes {
        let Meta::List(meta) = attr.meta else {
            continue;
        };

        if !meta.path.is_ident("utility_macros") {
            continue;
        }

        let mut tokens = meta.tokens.into_iter().peekable();
        let attr = parse_container_attributes_with_keys_tokens(&mut tokens);
        container_attributes.push(attr);
    }

    container_attributes
}
