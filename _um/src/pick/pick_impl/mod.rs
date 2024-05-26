mod pick_impl_struct_fields_named;
use pick_impl_struct_fields_named::impl_struct_fields_named;

mod pick_impl_enum;
use pick_impl_enum::impl_enum;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, Data, DataEnum, DeriveInput, Fields, FieldsNamed, Ident, Token, Variant,
};

use crate::attributes::{parse_container_attributes_with_keys, ContainerAttributesWithKeys};

pub enum ImplData {
    StructFieldsNamed(FieldsNamed),
    Enum(Punctuated<Variant, Token![,]>),
}

type ImplFn = fn(&ContainerAttributesWithKeys, &ImplData, &Ident) -> TokenStream;

pub fn pick_impl(
    DeriveInput {
        attrs,
        data,
        ident: type_ident,
        ..
    }: DeriveInput,
) -> TokenStream {
    let container_attrs = parse_container_attributes_with_keys(attrs);

    let (impl_f, data): (ImplFn, _) = match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => (
                impl_struct_fields_named,
                ImplData::StructFieldsNamed(fields),
            ),
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => panic!("Derive `Pick` is not supported for unit structs"),
        },
        Data::Enum(DataEnum { variants, .. }) => (impl_enum, ImplData::Enum(variants)),
        Data::Union(_) => panic!("Derive `Pick` is not supported for unions"),
    };

    let impls = container_attrs
        .iter()
        .map(|attr| impl_f(attr, &data, &type_ident));

    quote! {
        #(#impls)*
    }
}
