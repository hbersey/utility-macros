use syn::{parse_quote, Type};

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
