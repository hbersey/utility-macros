// #[utility_macros(name="Name")]
// #[utility_macros(case="UPPER_SNAKE")]
// #[utility_macros(skip)]

use convert_case::{Case, Casing as _};
use syn::{Attribute, Ident, Meta};

use crate::{
    error::{Error, Result},
    expect_token::expect_token,
    utils::CaseExt as _,
};

pub enum FieldVariantAttribute {
    Name(String),
    Case(Case),
    Skip,
}

pub fn parse_field_variant_attribute(
    tokens: &mut std::iter::Peekable<impl Iterator<Item = proc_macro2::TokenTree>>,
) -> Result<FieldVariantAttribute> {
    let ident = expect_token!(tokens, ident);

    match ident.to_string().as_str() {
        // #[utility_macros(name="Name")]
        "name" => {
            expect_token!(tokens, punct = '=');
            let name = expect_token!(tokens, string);
            Ok(FieldVariantAttribute::Name(name))
        }
        // #[utility_macros(case="UPPER_SNAKE")]
        "case" => {
            expect_token!(tokens, punct = '=');
            let case = expect_token!(tokens, string);
            Ok(FieldVariantAttribute::Case(
                Case::from_str(&case.to_string()).unwrap(),
            ))
        }
        // #[utility_macros(skip)]
        "skip" => Ok(FieldVariantAttribute::Skip),
        s => Err(Error::InvalidAttributeItem(s.to_string())),
    }
}

pub trait FieldVariantAttributes {
    fn name(&self) -> Option<&str>;
    fn case(&self) -> Option<Case>;

    fn ident(&self, ident: Ident) -> Ident {
        if let Some(name) = self.name() {
            Ident::new(name, ident.span())
        } else if let Some(case) = self.case() {
            let name = ident.to_string().to_case(case);
            Ident::new(&name, ident.span())
        } else {
            ident
        }
    }

    fn skip(&self) -> bool;
}

impl FieldVariantAttributes for Vec<FieldVariantAttribute> {
    fn name(&self) -> Option<&str> {
        self.iter()
            .filter_map(|attr| match attr {
                FieldVariantAttribute::Name(name) => Some(name.as_str()),
                _ => None,
            })
            .next()
    }

    fn case(&self) -> Option<Case> {
        self.iter()
            .filter_map(|attr| match attr {
                FieldVariantAttribute::Case(case) => Some(*case),
                _ => None,
            })
            .next()
    }

    fn skip(&self) -> bool {
        self.iter()
            .any(|attr| matches!(attr, FieldVariantAttribute::Skip))
    }
}

pub fn parse_field_variant_attributes(
    type_attrs: Vec<Attribute>,
) -> Result<Vec<FieldVariantAttribute>> {
    let mut field_variant_attrs = Vec::new();

    for attr in type_attrs {
        let Meta::List(meta) = attr.meta else {
            continue;
        };

        if !meta.path.is_ident("utility_macros") {
            continue;
        }

        let mut tokens = meta.tokens.into_iter().peekable();
        let attr = parse_field_variant_attribute(&mut tokens)?;
        field_variant_attrs.push(attr);
    }

    Ok(field_variant_attrs)
}
