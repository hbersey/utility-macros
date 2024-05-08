use proc_macro2::TokenTree;
use syn::{Field, Ident, Meta};

pub struct FieldAttributesData {
    pub ident: Ident,
    pub skip: bool,
}

pub fn field_attributes(helper: &'static str, field: &Field) -> FieldAttributesData {
    let mut field_ident = field.ident.clone().expect("Field must have an identifier");
    let mut skip = false;

    for attr in &field.attrs {
        let Meta::List(meta) = &attr.meta else {
            continue;
        };

        if !meta
            .path
            .segments
            .first()
            .map_or(false, |seg| seg.ident == helper)
        {
            continue;
        }

        let mut tokens = meta.clone().tokens.into_iter().peekable();
        while let Some(token) = tokens.next() {
            let TokenTree::Ident(ident) = token else {
                continue;
            };

            if ident == "name" {
                let Some(TokenTree::Punct(punct)) = tokens.next() else {
                    panic!("Expected punct")
                };

                if punct.as_char() != '=' {
                    panic!("Expected '='")
                }

                let Some(TokenTree::Ident(ident)) = tokens.next() else {
                    panic!("Expected ident")
                };

                field_ident = ident;
            } else if ident == "skip" {
                skip = true;
            }
        }
    }

    FieldAttributesData {
        ident: field_ident,
        skip,
    }
}
