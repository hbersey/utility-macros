use syn::{parse_quote, PathArguments, Type};

pub fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            let path = &type_path.path;
            let segments = &path.segments;
            if segments.len() == 1 {
                let segment = &segments[0];
                if segment.ident == "Option" {
                    return true;
                }
            }
        }
        _ => {}
    }
    false
}

pub fn as_option(ty: &Type) -> Type {
    if is_option(ty) {
        return ty.clone();
    }

    parse_quote!(Option<#ty>)
}

pub fn as_required(ty: &Type) -> Type {
    if !is_option(ty) {
        return ty.clone();
    }

    let Type::Path(ty) = ty else {
        panic!("Expected Type::Path")
    };

    let segments = &ty.path.segments;
    if segments.len() != 1 {
        panic!("Expected single segment")
    }

    let PathArguments::AngleBracketed(args) = &segments[0].arguments else {
        panic!("Expected angle bracketed arguments")
    };

    let required_ty = args.args.iter().next().expect("Expected argument");
    parse_quote!(#required_ty)
}
