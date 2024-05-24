use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field};

use crate::{
    attributes::{
        parse_container_attributes, parse_field_variant_attributes, ContainerAttributes as _,
        FieldVariantAttributes as _,
    },
    utils::{ResultExt as _, TypeExt as _},
};

pub fn required_impl(
    DeriveInput {
        attrs,
        ident: type_ident,
        data,
        ..
    }: DeriveInput,
) -> TokenStream {
    let attrs = parse_container_attributes(attrs).or_panic();

    let Data::Struct(data) = data else {
        panic!("Expected struct")
    };

    let mut static_assertions = Vec::new();
    let mut struct_body = Vec::new();
    let mut to_required_body = Vec::new();
    let mut to_type_body = Vec::new();
    let mut impl_partial_eq = true;
    let mut partial_eq = Vec::new();

    for field in &data.fields {
        let Field {
            vis,
            ty,
            ident: type_ident,
            ..
        } = field;

        let type_ident = type_ident.clone().expect("Expected ident");
        let required_ident = attrs.field_ident(type_ident.clone());
        let field_attrs = parse_field_variant_attributes(field.attrs.clone()).or_panic();
        let required_ident = field_attrs.ident(required_ident);

        if field_attrs.skip() {
            if ty.is_option() {
                to_type_body.push(quote! {
                    #type_ident: None,
                });
                partial_eq.push(quote! {
                    self.#type_ident.is_none()
                });
            } else {
                static_assertions.push(quote! {
                    ::utility_macros::_um::_sa::assert_impl_all!(#ty: Default);
                });
                to_type_body.push(quote! {
                    #type_ident: Default::default(),
                });
                impl_partial_eq = false;
            }
            continue;
        }

        let required_ty = ty.as_required();
        struct_body.push(quote! {
            #vis #required_ident: #required_ty,
        });

        static_assertions.push(quote! {
            ::utility_macros::_um::_sa::assert_impl_all!(#required_ty: Clone);
        });

        if ty.is_option() {
            to_required_body.push(quote! {
                #required_ident: self.#type_ident.clone().ok_or_else(|| ::utility_macros::_um::error::Error::MissingField(stringify!(#required_ident)))?,
            });
            to_type_body.push(quote! {
                #type_ident: Some(self.#required_ident.clone()),
            });
            partial_eq.push(quote! {
                self.#type_ident.clone().map_or(false, |val| val == other.#required_ident)
            });
        } else {
            to_required_body.push(quote! {
                #required_ident: self.#type_ident.clone(),
            });
            to_type_body.push(quote! {
                #type_ident: self.#required_ident.clone(),
            });
            partial_eq.push(quote! {
                self.#type_ident == other.#required_ident
            });
        }
    }

    let required_ident = attrs.ident(format_ident!("Required{}", type_ident));

    let derive_statement = attrs.derive_statement("PartialEq, Clone, Debug").or_panic();

    let partial_eq_impl = if impl_partial_eq {
        quote! {
            impl PartialEq<#required_ident> for #type_ident {
                fn eq(&self, other: &#required_ident) -> bool {
                    #(#partial_eq)&& *
                }
            }
        }
    } else {
        quote! {
            impl PartialEq<#required_ident> for #type_ident {
                fn eq(&self, _: &#required_ident) -> bool {
                    panic!("Partial equality can't be implemented for types that have skipped, non-optional fields");
                }
            }
        }
    };

    quote! {
        #(#static_assertions)*

        #derive_statement
        pub struct #required_ident {
            #(#struct_body)*
        }

        impl ::utility_macros::_um::required::HasRequired for #type_ident {
            type Required = #required_ident;

            fn required(&self) -> ::utility_macros::_um::error::Result<Self::Required> {
                Ok(Self::Required {
                    #(#to_required_body)*
                })
            }
        }

        impl ::utility_macros::_um::required::Required for #required_ident {
            type Type = #type_ident;

            fn type_(&self) -> Self::Type {
                #type_ident {
                    #(#to_type_body)*
                }
            }
        }

        impl TryFrom<#type_ident> for #required_ident {
            type Error = ::utility_macros::_um::error::Error;

            fn try_from(value: #type_ident) -> ::utility_macros::_um::error::Result<Self> {
                ::utility_macros::_um::required::HasRequired::required(&value)
            }
        }

        impl From<#required_ident> for #type_ident {
            fn from(required: #required_ident) -> Self {
                ::utility_macros::_um::required::Required::type_(&required)
            }
        }

       #partial_eq_impl
    }
}
