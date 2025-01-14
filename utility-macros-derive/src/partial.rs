use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Delimiter, Ident, TokenStream, TokenTree};
use quote::quote;
use syn::{punctuated::Punctuated, Field, Fields, FieldsNamed, Generics, ItemStruct, Token};

use crate::utils::TypeExt;

pub fn partial_inner(attr: TokenStream1, item: TokenStream1) -> Result<TokenStream1, TokenStream1> {
    let full_struct = syn::parse::<ItemStruct>(item.clone()).map_err(|_| item)?;

    let ParseAttrRes {
        partial_ident,
        partial_derives,
    } = parse_attr(attr)?;

    let partial_derives = if partial_derives.is_empty() {
        quote! {}
    } else {
        quote! {
            #[derive(#(#partial_derives),*)]
        }
    };

    let partial_struct = generate_partial_struct(&full_struct, &partial_ident)?;

    let has_parital_impl = generate_has_partial_impl(
        &full_struct.generics,
        &full_struct.ident,
        &partial_ident,
        &full_struct.fields,
    );

    let from_full_for_partial =
        generate_from_full_for_partial(&full_struct.generics, &full_struct.ident, &partial_ident);

    let partial_trait_impl = generate_partial_impl(
        &full_struct.generics,
        &full_struct.ident,
        &partial_ident,
        &full_struct.fields,
    );

    let try_from_partial_for_full = generate_try_from_partial_for_full(
        &full_struct.generics,
        &full_struct.ident,
        &partial_ident,
    );

    let partial_eq_impl = generate_partial_eq_impl(
        &full_struct.generics,
        &full_struct.ident,
        &partial_ident,
        &full_struct.fields,
    );

    let output = quote::quote! {
        #full_struct

        #has_parital_impl
        #from_full_for_partial

        #partial_derives
        #partial_struct

        #partial_trait_impl
        #try_from_partial_for_full

        #partial_eq_impl
    };

    Ok(output.into())
}

struct ParseAttrRes {
    partial_ident: syn::Ident,
    partial_derives: Vec<syn::Ident>,
}

fn parse_attr(attr: TokenStream1) -> Result<ParseAttrRes, TokenStream1> {
    let attr = TokenStream::from(attr);
    let mut tokens = attr.into_iter();

    let Some(TokenTree::Ident(partial_ident)) = tokens.next() else {
        return Err(quote! {
            compile_error!("Expected an identifier");
        }
        .into());
    };

    let mut partial_derives = Vec::new();
    if let Some(TokenTree::Punct(punct)) = tokens.next() {
        if punct.as_char() != ',' {
            return Err(quote! {
                compile_error!("Expected a comma");
            }
            .into());
        }

        match tokens.next() {
            Some(ident) if ident.to_string() == "derive" => {
                let group = match tokens.next() {
                    Some(TokenTree::Group(group))
                        if group.delimiter() == Delimiter::Parenthesis =>
                    {
                        group
                    }
                    _ => {
                        return Err(quote! {
                            compile_error!("Expected a `(...)`");
                        }
                        .into());
                    }
                };

                let mut group = group.stream().into_iter();
                while let Some(TokenTree::Ident(ident)) = group.next() {
                    partial_derives.push(ident);
                    match group.next() {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {}
                        None => {}
                        _ => {
                            return Err(quote! {
                                compile_error!("Expected a comma or closing parenthesis");
                            }
                            .into());
                        }
                    }
                }
            }
            _ => {
                return Err(quote! {
                    compile_error!("Expected 'derive'");
                }
                .into());
            }
        }
    }

    Ok(ParseAttrRes {
        partial_ident,
        partial_derives,
    })
}

fn generate_partial_struct(
    full_struct: &ItemStruct,
    partial_ident: &syn::Ident,
) -> Result<ItemStruct, TokenStream1> {
    let full_fields = &full_struct.fields;

    let mut partial_fields = Punctuated::<Field, Token![,]>::new();
    for field in full_fields.iter().cloned() {
        if (&field.ty).is_optional() {
            partial_fields.push(field.clone());
            continue;
        }

        let partial_field = Field {
            ty: field.ty.make_optional(),
            ..field
        };

        partial_fields.push(partial_field);
    }

    let partial_fields = Fields::Named(FieldsNamed {
        brace_token: Default::default(),
        named: partial_fields,
    });

    Ok(ItemStruct {
        ident: partial_ident.clone(),
        fields: partial_fields,
        ..full_struct.clone()
    })
}

fn generate_has_partial_impl(
    generics: &Generics,
    full_ident: &Ident,
    partial_ident: &Ident,
    full_fields: &Fields,
) -> TokenStream {
    let inner = full_fields.iter().map(|field| {
        let f_ident = &field.ident;
        if field.ty.is_optional() {
            quote! {
                #f_ident: self.#f_ident
            }
        } else {
            quote! {
                #f_ident: Some(self.#f_ident)
            }
        }
    });

    quote! {
        impl #generics utility_macros::HasPartial for #full_ident #generics {
            type Partial = #partial_ident #generics;

            fn as_partial(self) -> Self::Partial {
                Self::Partial {
                    #(#inner,)*
                }
            }
        }
    }
}

fn generate_from_full_for_partial(
    generics: &Generics,
    full_ident: &Ident,
    partial_ident: &Ident,
) -> TokenStream {
    quote! {
        impl #generics From<#full_ident #generics> for #partial_ident #generics {
            fn from(full: #full_ident #generics) -> Self {
                utility_macros::HasPartial::as_partial(full)
            }
        }
    }
}


fn generate_partial_impl(
    generics: &Generics,
    full_ident: &Ident,
    partial_ident: &Ident,
    full_fields: &Fields,
) -> TokenStream {
    let inner = full_fields.iter().map(|field| {
        let f_ident = &field.ident;
        if field.ty.is_optional() {
            quote! {
                #f_ident: self.#f_ident
            }
        } else {
            quote! {
                #f_ident: self.#f_ident.ok_or_else(|| utility_macros::Error::MissingField(stringify!(#f_ident)))?
            }
        }
    });

    quote! {
        impl #generics utility_macros::Partial for #partial_ident #generics {
            type Full = #full_ident #generics;

            fn try_as_full(self) -> utility_macros::Result<Self::Full> {
                Ok(Self::Full {
                    #(#inner,)*
                })
            }
        }
    }
}

fn generate_try_from_partial_for_full(
    generics: &Generics,
    full_ident: &Ident,
    partial_ident: &Ident,
) -> TokenStream {
    quote! {
        impl #generics TryFrom<#partial_ident #generics> for #full_ident #generics {
            type Error = utility_macros::Error;

            fn try_from(partial: #partial_ident #generics) -> utility_macros::Result<Self> {
                utility_macros::Partial::try_as_full(partial)
            }
        }
    }
}

fn generate_partial_eq_impl(
    generics: &Generics,
    full_ident: &Ident,
    partial_ident: &Ident,
    full_fields: &Fields,
) -> TokenStream {
    let inner = full_fields.iter().map(|field| {
        let f_ident = &field.ident;
        let f_ty = &field.ty;

        if f_ty.is_optional() {
            quote! {
                self.#f_ident.as_ref() == other.#f_ident.as_ref()
            }
        } else {
            quote! {
                self.#f_ident.as_ref() == Some(&other.#f_ident)
            }
        }
    });

    if generics.params.is_empty() {
        return quote! {
            impl PartialEq<#full_ident> for #partial_ident {
                fn eq(&self, other: &#full_ident) -> bool {
                    #(#inner)&&*
                }
            }
        };
    }

    let g_params = generics.clone().params.into_iter();
    let where_statement = quote! {
        where #(#g_params: PartialEq<#g_params>),*
    };

    quote! {
        impl #generics PartialEq<#full_ident #generics> for #partial_ident #generics #where_statement{
            fn eq(&self, other: &#full_ident #generics) -> bool {
                #(#inner)&&*
            }
        }
    }
}
