use proc_macro2::TokenTree;
use syn::{Attribute, Ident, Meta};

pub struct ContainerAttributesData {
    pub ident: Ident,
    pub derives: Vec<Ident>,
}

pub fn container_attributes(
    helper: &'static str,
    attributes: Vec<Attribute>,
    or_ident: Ident,
) -> ContainerAttributesData {
    let mut field_ident = or_ident;
    let mut derives = Vec::new();

    for attr in attributes {
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

            match ident.to_string().as_str() {
                "name" => {
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
                }
                "derive" => {
                    // ignore equals sign
                    if let Some(TokenTree::Punct(punct)) = tokens.peek() {
                        if punct.as_char() != '=' {
                            panic!("Expected '=' ident or group of identifiers")
                        }

                        tokens.next();
                    }

                    let Some(TokenTree::Group(group)) = tokens.next() else {
                        panic!("Expected group")
                    };

                    let mut tokens = group.stream().into_iter().peekable();
                    while let Some(token) = tokens.next() {
                        let TokenTree::Ident(ident) = token else {
                            panic!("Expected ident")
                        };

                        derives.push(ident);

                        if let Some(TokenTree::Punct(_)) = tokens.peek() {
                            tokens.next();
                        }
                    }
                }
                _ => {}
            }
        }
    }

    ContainerAttributesData {
        ident: field_ident,
        derives,
    }
}
