use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Type, TypePath};

#[proc_macro_derive(Partial)]
pub fn partial(input: TokenStream) -> TokenStream {
    let DeriveInput {
        vis,
        ident: original_ident,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let Data::Struct(data) = data else {
        panic!("`#[partial]` can only be used on structs")
    };

    let partial_ident = format_ident!("{}Partial", original_ident);

    let mut partial_fields = Vec::new();
    let mut to_original_fields = Vec::new();
    let mut to_partial_fields = Vec::new();
    let mut partial_eq = Vec::new();

    for field in &data.fields {
        let ident = &field.ident;

        if let Type::Path(TypePath { path, .. }) = &field.ty {
            if path
                .segments
                .first()
                .map_or(false, |segment| segment.ident == "Option")
            {
                partial_fields.push(field.clone());

                to_original_fields.push(quote! {
                    #ident: self.#ident.clone()
                });

                to_partial_fields.push(quote! {
                    #ident: self.#ident.clone()
                });

                partial_eq.push(quote! {
                    self.#ident == other.#ident
                });

                continue;
            }
        }

        partial_fields.push({
            let mut f = field.clone();
            let ty = &f.ty;
            f.ty = parse_quote!(Option<#ty>);
            f
        });

        to_original_fields.push(quote! {
            #ident: self.#ident.clone().ok_or_else(|| _utility_macros::Error::MissingField(stringify!(#ident)))?
        });

        to_partial_fields.push(quote! {
            #ident: Some(self.#ident.clone())
        });

        partial_eq.push(quote! {
            self.#ident.clone().map_or(false, |val| val == other.#ident)
        })
    }

    quote! {
        #[derive(Clone, Debug, PartialEq)]
        #vis struct #partial_ident {
            #(#partial_fields),*
        }

        impl _utility_macros::HasPartial for #original_ident {
            type Partial = #partial_ident;

            fn partial(&self) -> #partial_ident {
                #partial_ident {
                    #(#to_partial_fields),*
                }
            }
        }

        impl From<#original_ident> for #partial_ident {
            fn from(value: #original_ident) -> Self {
                _utility_macros::HasPartial::partial(&value)
            }
        }

        impl _utility_macros::Partial for #partial_ident {
            type Full = #original_ident;

            fn full(&self) -> _utility_macros::Result<#original_ident> {
                Ok(#original_ident {
                    #(#to_original_fields),*
                })
            }
        }

        impl TryFrom<#partial_ident> for #original_ident {
            type Error = _utility_macros::Error;

            fn try_from(value: #partial_ident) -> _utility_macros::Result<Self> {
                value.full()
            }
        }

        impl PartialEq<#original_ident> for #partial_ident {
            fn eq(&self, other: &#original_ident) -> bool {
                #(#partial_eq)&&*
            }
        }
    }
    .into()
}
