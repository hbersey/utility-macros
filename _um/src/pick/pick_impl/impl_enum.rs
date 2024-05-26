use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, FieldsNamed, Ident, Variant};

use super::ImplData;
use crate::{
    attributes::{
        parse_field_variant_attributes, ContainerAttributesWithKeys, FieldVariantAttributes as _,
    },
    utils::ResultExt as _,
};

pub(crate) fn impl_enum(
    attr: &ContainerAttributesWithKeys,
    variants: &ImplData,
    type_ident: &Ident,
) -> TokenStream {
    let all_variants = match variants {
        ImplData::Enum(variants) => variants,
        _ => unreachable!("Expected `Enum`."),
    };

    let pick_ident: Ident = attr.ident();

    let mut type_variants = Vec::new();
    let mut type_idents = Vec::new();

    let mut pick_variants = Vec::new();
    let mut pick_idents = Vec::new();
    let mut remaining_variants: Vec<&Variant> = Vec::new();

    let mut pick_fn_body = Vec::new();

    for type_variant in all_variants.iter() {
        let field_attr = parse_field_variant_attributes(type_variant.attrs.clone()).or_panic();

        let type_variant_ident = &type_variant.ident;

        if attr.keys().contains(&type_variant_ident.to_string()) {
            type_variants.push(type_variant);
            type_idents.push(type_variant_ident);

            let pick_variant_ident = {
                let mut ident = attr.field_variant_ident(type_variant_ident.clone());
                ident = field_attr.ident(ident);
                ident
            };

            pick_idents.push(pick_variant_ident.clone());

            pick_variants.push({
                let mut variant = type_variant.clone();
                variant.ident = pick_variant_ident.clone();
                variant
            });

            match &type_variant.fields {
                Fields::Named(FieldsNamed { named, .. }) => {
                    let field_idents = named
                        .iter()
                        .map(|field| {
                            field
                                .ident
                                .as_ref()
                                .expect("Expected field to have an identifier")
                        })
                        .collect::<Vec<_>>();

                    pick_fn_body.push(quote!{
                        #type_ident::#type_variant_ident { #(#field_idents),* } => Ok(#pick_ident::#pick_variant_ident { #(#field_idents: #field_idents.clone()),* }),
                        _ => Err(::utility_macros::_um::error::Error::InvalidVariant(stringify!(#type_variant_ident).to_string())),
                    })
                }
                Fields::Unnamed(_) => {
                    let idents = (0..type_variant.fields.len())
                        .map(|i| Ident::new(&format!("field{}", i), type_variant_ident.span()))
                        .collect::<Vec<_>>();

                    pick_fn_body.push(quote!{
                        #type_ident::#type_variant_ident(#(#idents),*) => Ok(#pick_ident::#pick_variant_ident(#(#idents.clone()),*)),
                        _ => Err(::utility_macros::_um::error::Error::InvalidVariant(stringify!(#type_variant_ident).to_string())),
                    })
                }
                Fields::Unit => {
                    pick_fn_body.push(quote!{
                        #type_ident::#type_variant_ident => Ok(#pick_ident::#pick_variant_ident),
                        _ => Err(::utility_macros::_um::error::Error::InvalidVariant(stringify!(#type_variant_ident).to_string())),
                    })
                }
            }
        } else {
            remaining_variants.push(type_variant);
        }
    }

    quote! {
        pub enum #pick_ident {
            #(#pick_variants),*
        }

        impl ::utility_macros::_um::pick::HasPick for #type_ident {
            type Pick = #pick_ident;
        }

        impl ::utility_macros::_um::pick::HasPickEnum for #type_ident {
            fn pick(&self) -> ::utility_macros::_um::error::Result<Self::Pick> {
                match self {
                    #(#pick_fn_body)*
                }
            }
        }

        impl ::utility_macros::_um::pick::Pick for #pick_ident {
            type Type = #type_ident;
        }

        impl TryFrom<#type_ident> for #pick_ident {
            type Error = ::utility_macros::_um::error::Error;

            fn try_from(value: #type_ident) -> ::utility_macros::_um::error::Result<Self> {
                ::utility_macros::_um::pick::HasPickEnum::pick(&value)
            }
        }
    }
}
