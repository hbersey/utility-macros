use syn::{Type, TypePath};

pub trait TypeExt {
    fn is_optional(&self) -> bool;
    fn make_optional(self) -> Type;

    fn is_required(&self) -> bool {
        !self.is_optional()
    }

    fn make_required(self) -> Type;
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
        if self.is_optional() {
            return self;
        }

        let path = syn::parse_quote! { Option<#self> };
        syn::Type::Path(syn::TypePath { qself: None, path })
    }

    fn make_required(self) -> Type {
        if self.is_required() {
            return self;
        }

        if let Type::Path(TypePath { path, .. }) = self.clone() {
            if path.segments.len() != 1 {
                return self;
            }

            if let Some(segment) = path.segments.first() {
                if segment.ident == "Option" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if args.args.len() == 1 {
                            if let syn::GenericArgument::Type(ty) = args.args.first().unwrap() {
                                return ty.clone();
                            }
                        }
                    }
                }
            }
        }

        panic!("Expected an Option<T> type")
    }
}
