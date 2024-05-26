use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::ImplData;
use crate::{
    attributes::{parse_field_variant_attributes, ContainerAttributesWithKeys, FieldVariantAttributes as _},
    utils::ResultExt as _,
};

pub(crate) fn impl_struct_fields_named(
    attr: &ContainerAttributesWithKeys,
    fields: &ImplData,
    type_ident: &Ident,
) -> TokenStream {
    let ImplData::StructFieldsNamed(all_fields) = fields else {
        unreachable!("Expected `StructFieldsNamed`.");
    };

    let mut static_assertions = Vec::new();

    let mut type_fields = Vec::new();
    let mut type_idents = Vec::new();

    let mut pick_fields = Vec::new();
    let mut pick_idents = Vec::new();
    let mut remaining_fields = Vec::new();

    for type_field in all_fields.named.iter() {
        let field_attr = parse_field_variant_attributes(type_field.attrs.clone()).or_panic();

        let type_field_ident = type_field
            .ident
            .as_ref()
            .expect("Expected field to have an identifier");

        if attr.keys().contains(&type_field_ident.to_string()) {
            let field_ty = &type_field.ty;
            static_assertions.push(quote! {
                ::utility_macros::_um::_sa::assert_impl_all!(#field_ty: Clone);
            });

            type_idents.push(type_field_ident.clone());
            type_fields.push(type_field);

            let mut pick_field_ident = attr.field_variant_ident(type_field_ident.clone());
            pick_field_ident = field_attr.ident(pick_field_ident);
            pick_idents.push(pick_field_ident.clone());

            pick_fields.push({
                let mut field = type_field.clone();
                field.ident = Some(field_attr.ident(type_field_ident.clone()));
                field
            });
        } else {
            remaining_fields.push(type_field);
        }
    }

    if attr.keys().len() != type_fields.len() {
        panic!("Expected all keys to be present in the struct");
    }

    let ident = attr.ident();

    quote! {
        #(#static_assertions)*

        pub struct #ident {
            #(#type_fields),*
        }

        impl ::utility_macros::_um::pick::HasPick for #type_ident {
            type Pick = #ident;
        }
        
        impl ::utility_macros::_um::pick::HasPickStruct for #type_ident {
            fn pick(&self) -> Self::Pick {
                #ident {
                    #(#type_idents: self.#pick_idents.clone()),*
                }
            }
        }

        impl ::utility_macros::_um::pick::Pick for #ident {
            type Type = #type_ident;
        }

        impl From<#type_ident> for #ident {
            fn from(value: #type_ident) -> Self {
                ::utility_macros::_um::pick::HasPickStruct::pick(&value)
            }
        }
    }
}
