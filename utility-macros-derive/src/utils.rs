use syn::{Type, TypePath};

pub trait TypeExt {
    fn is_optional(&self) -> bool;
    fn make_optional(self) -> Type;
}

impl TypeExt for Type {
    fn is_optional(&self) -> bool {
        match self {
            Type::Path(TypePath { path, .. }) => {
                if path.segments.len() != 1 {
                    return false;
                }

                if let Some(segment) = path.segments.first() {
                    segment.ident == "Option"
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn make_optional(self) -> Type {
        let path = syn::parse_quote! { Option<#self> };
        syn::Type::Path(syn::TypePath { qself: None, path })
    }
}
