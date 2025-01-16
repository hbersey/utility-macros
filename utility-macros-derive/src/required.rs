use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Delimiter, Ident, TokenStream, TokenTree};
use quote::quote;
use syn::{punctuated::Punctuated, Field, Fields, FieldsNamed, Generics, ItemStruct};

use crate::utils::TypeExt as _;

pub fn required_inner(
    attr: TokenStream1,
    item: TokenStream1,
) -> Result<TokenStream1, TokenStream1> {
    let full_struct = syn::parse::<ItemStruct>(item.clone()).map_err(|_| item)?;

    let ParseAttrRes {
        required_ident,
        required_derives,
    } = parse_attr(attr)?;

    let required_derives = if required_derives.is_empty() {
        quote! {}
    } else {
        quote! {
            #[derive(#(#required_derives),*)]
        }
    };

    let required_struct = generate_required_struct(&full_struct, &required_ident)?;

    let has_required_impl = generate_has_required_impl(
        &full_struct.generics,
        &full_struct.ident,
        &required_ident,
        &full_struct.fields,
    );

    let try_from_not_required_for_required = generate_try_from_not_required_for_required(
        &full_struct.generics,
        &full_struct.ident,
        &required_ident,
    );

    let required_trait_impl = generate_required_impl(
        &full_struct.generics,
        &full_struct.ident,
        &required_ident,
        &full_struct.fields,
    );

    let from_required_for_not_required = generate_from_required_for_not_required(
        &full_struct.generics,
        &full_struct.ident,
        &required_ident,
    );

    let partial_eq_impl = generate_partial_eq_impl(
        &full_struct.generics,
        &full_struct.ident,
        &required_ident,
        &full_struct.fields,
    );

    let output = quote! {
        #full_struct

        #has_required_impl
        #try_from_not_required_for_required

        #required_derives
        #required_struct

        #required_trait_impl
        #from_required_for_not_required

        #partial_eq_impl
    };

    Ok(output.into())
}

pub struct ParseAttrRes {
    pub required_ident: Ident,
    pub required_derives: Vec<Ident>,
}

fn parse_attr(attr: TokenStream1) -> Result<ParseAttrRes, TokenStream1> {
    let attr = TokenStream::from(attr);
    let mut tokens = attr.into_iter();

    let Some(TokenTree::Ident(required_ident)) = tokens.next() else {
        return Err(quote! {
            compile_error!("Expected an identifier");
        }
        .into());
    };

    let mut required_derives = Vec::new();
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
                    required_derives.push(ident);
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
        required_ident,
        required_derives,
    })
}

fn generate_required_struct(
    full_struct: &ItemStruct,
    required_ident: &Ident,
) -> Result<ItemStruct, TokenStream1> {
    let mut required_fields = Punctuated::new();
    for field in full_struct.fields.iter().cloned() {
        if field.ty.is_required() {
            required_fields.push(field.clone());
            continue;
        }

        let required_ty = field.clone().ty.make_required();
        let required_field = Field {
            ty: required_ty,
            ..field.clone()
        };

        required_fields.push(required_field);
    }

    let required_struct = ItemStruct {
        ident: required_ident.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named: required_fields,
        }),
        ..full_struct.clone()
    };

    Ok(required_struct)
}

fn generate_has_required_impl(
    generics: &Generics,
    not_required_ident: &Ident,
    required_ident: &Ident,
    not_required_fields: &Fields,
) -> TokenStream {
    let inner = not_required_fields.iter().map(|field| {
        let field_ident = &field.ident;

        if field.ty.is_required() {
             quote! {
                #field_ident: self.#field_ident
            }
        } else {
            quote! {
                #field_ident: self.#field_ident.ok_or_else(|| utility_macros::Error::MissingField(stringify! (#field_ident)))?
            }
        }
    });

    quote! {
        impl #generics utility_macros::HasRequired for #not_required_ident #generics {
            type Required = #required_ident #generics;

            fn try_as_required(self) -> utility_macros::Result<Self::Required> {
                Ok(#required_ident {
                    #(#inner),*
                })
            }
        }
    }
}

fn generate_try_from_not_required_for_required(
    generics: &Generics,
    not_required_ident: &Ident,
    required_ident: &Ident,
) -> TokenStream {
    quote! {
        impl #generics TryFrom<#not_required_ident #generics> for #required_ident #generics {
            type Error = utility_macros::Error;

            fn try_from(value: #not_required_ident #generics) -> utility_macros::Result<Self> {
                utility_macros::HasRequired::try_as_required(value)
            }
        }
    }
}

fn generate_required_impl(
    generics: &Generics,
    not_required_ident: &Ident,
    required_ident: &Ident,
    not_required_fields: &Fields,
) -> TokenStream {
    let inner = not_required_fields.iter().map(|field| {
        let field_ident = &field.ident;
        let field_ty = &field.ty;

        if field_ty.is_required() {
            quote! {
                #field_ident: self.#field_ident
            }
        } else {
            quote! {
                #field_ident: Some(self.#field_ident)
            }
        }
    });

    quote! {
        impl #generics utility_macros::Required for #required_ident #generics {
            type NotRequired = #not_required_ident #generics;

            fn as_not_required(self) -> Self::NotRequired {
                #not_required_ident {
                    #(#inner),*
                }
            }
        }
    }
}

fn generate_from_required_for_not_required(
    generics: &Generics,
    not_required_ident: &Ident,
    required_ident: &Ident,
) -> TokenStream {
    quote! {
        impl #generics From<#required_ident #generics> for #not_required_ident #generics {
            fn from(value: #required_ident #generics) -> Self {
                utility_macros::Required::as_not_required(value)
            }
        }
    }
}

fn generate_partial_eq_impl(
    generics: &Generics,
    not_required_ident: &Ident,
    required_ident: &Ident,
    not_required_fields: &Fields,
) -> TokenStream {
    let inner = not_required_fields.iter().map(|field| {
        let field_ident = &field.ident;
        let field_ty = &field.ty;

        if field_ty.is_required() {
            quote! {
                &self.#field_ident == &other.#field_ident
            }
        } else {
            quote! {
                match &self.#field_ident {
                    None => false,
                    Some(#field_ident) => #field_ident == &other.#field_ident,
                }
            }
        }
    });

    if generics.params.is_empty() {
        quote! {
            impl PartialEq<#required_ident> for #not_required_ident {
                fn eq(&self, other: &Self) -> bool {
                    #(#inner)&&*
                }
            }
        }
    } else {
        let g_params = generics.clone().params.into_iter();
        let where_statment = quote! {
            where #(#g_params: PartialEq<#g_params>, #g_params: AsRef<#g_params>),*
        };
        quote! {
            impl #generics PartialEq<#required_ident #generics> for #not_required_ident #generics #where_statment {
                fn eq(&self, other: &#required_ident #generics) -> bool {
                    #(#inner)&&*
                }
            }
        }
    }
}
