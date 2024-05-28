use core::panic;

use syn::{parse_quote, PathArguments, Type};

pub trait TypeExt {
    fn is_option(&self) -> bool;
    fn as_option(&self) -> Self;

    fn is_required(&self) -> bool {
        !self.is_option()
    }
    fn as_required(&self) -> Self;
}

impl TypeExt for Type {
    fn is_option(&self) -> bool {
        match self {
            Type::Path(type_path) => {
                let path = &type_path.path;
                let segments = &path.segments;
                if segments.len() == 1 {
                    let segment = &segments[0];
                    if segment.ident == "Option" {
                        return true;
                    }
                }
                false
            }
            _ => panic!("Expected Type::Path"),
        }
    }

    fn as_option(&self) -> Self {
        if self.is_option() {
            return self.clone();
        }

        parse_quote!(Option<#self>)
    }

    // Does this need so many panic!()s? Should really be using Result.
    fn as_required(&self) -> Self {
        match self {
            Type::Path(ty) => {
                if !self.is_option() {
                    return self.clone();
                }

                let segments = &ty.path.segments;

                let PathArguments::AngleBracketed(args) = &segments[0].arguments else {
                    panic!("Expected angle bracketed arguments")
                };
                let required_ty = args.args.iter().next().expect("Expected argument");
                parse_quote!(#required_ty)
            }
            _ => return self.clone(),
        }
    }
}
