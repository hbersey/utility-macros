use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field};

use crate::{
    attributes::{
        parse_container_attributes, parse_field_variant_attributes, ContainerAttributes,
        FieldVariantAttributes,
    },
    utils::{ResultExt as _, TypeExt as _},
};

pub fn readonly_impl(
    DeriveInput {
        attrs,
        ident: type_ident,
        data,
        ..
    }: DeriveInput,
) -> TokenStream {
    let container_attrs = parse_container_attributes(attrs).or_panic();
    let ro_container_ident = container_attrs.ident(type_ident.clone());

    let Data::Struct(data) = data else {
        panic!("Expected struct")
    };

    let mut static_assertions = Vec::new();
    let mut struct_body = Vec::new();
    let mut impl_getters = Vec::new();
    let mut impl_new_params = Vec::new();
    let mut impl_new_body = Vec::new();
    let mut to_readonly_body = Vec::new();
    let mut to_type_body = Vec::new();
    let mut partial_eq_conditions = Vec::new();
    let mut impl_partial_eq = true;

    for field in &data.fields {
        let Field {
            ty,
            ident: type_field_ident,
            ..
        } = field;

        let type_ident = type_field_ident.clone().expect("Expected ident");

        let field_attrs = parse_field_variant_attributes(field.attrs.clone()).or_panic();
        let ro_field_ident = {
            let ident = container_attrs.field_ident(type_ident.clone());
            field_attrs.ident(ident)
        };

        if field_attrs.skip() {
            if ty.is_option() {
                to_type_body.push(quote! {
                    #type_ident: None,
                });
                partial_eq_conditions.push(quote! {
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

        static_assertions.push(quote! {
            ::utility_macros::_um::_sa::assert_impl_all!(#ty: Clone);
        });

        struct_body.push(quote! {
            #ro_field_ident: #ty,
        });

        impl_new_params.push(quote! {
            #ro_field_ident: #ty
        });

        impl_new_body.push(quote! {
            #ro_field_ident
        });

        impl_getters.push(quote! {
            pub fn #ro_field_ident(&self) -> &#ty {
                &self.#type_ident
            }
        });

        to_readonly_body.push(quote! {
            #ro_field_ident: self.#type_ident.clone(),
        });

        to_type_body.push(quote! {
            #type_ident: self.#ro_field_ident.clone(),
        });

        partial_eq_conditions.push(quote! {
            self.#type_ident == other.#ro_field_ident
        });
    }

    let derive_statement = container_attrs
        .derive_statement("Clone, Debug, PartialEq")
        .or_panic();

    let partial_eq_impl = if impl_partial_eq {
        quote! {
            impl PartialEq<#ro_container_ident> for #type_ident {
                fn eq(&self, other: &#ro_container_ident) -> bool {
                    #(#partial_eq_conditions)&& *
                }
            }
        }
    } else {
        quote! {
            impl PartialEq<#ro_container_ident> for #type_ident {
                fn eq(&self, _: &#ro_container_ident) -> bool {
                    panic!("Partial equality can't be implemented for types that have skipped, non-optional fields");
                }
            }
        }
    };

    quote! {
        #(#static_assertions)*

        #derive_statement
        pub struct #ro_container_ident {
            #(#struct_body)*
        }

        impl #ro_container_ident {
            pub fn new(#(#impl_new_params),*) -> Self {
                Self {
                    #(#impl_new_body),*
                }
            }

            #(#impl_getters)*
        }

        impl ::utility_macros::_um::readonly::HasReadonly for #type_ident {
            type Readonly = #ro_container_ident;

            fn readonly(&self) -> Self::Readonly {
                Self::Readonly {
                    #(#to_readonly_body)*
                }
            }
        }

        impl ::utility_macros::_um::readonly::Readonly for #ro_container_ident {
            type Type = #type_ident;

            fn type_(&self) -> Self::Type {
                #type_ident {
                    #(#to_type_body)*
                }
            }
        }

        impl From<#type_ident> for #ro_container_ident {
            fn from(value: #type_ident) -> Self {
                ::utility_macros::_um::readonly::HasReadonly::readonly(&value)
            }
        }

        impl From<#ro_container_ident> for #type_ident {
            fn from(value: #ro_container_ident) -> Self {
                ::utility_macros::_um::readonly::Readonly::type_(&value)
            }
        }

        #partial_eq_impl
    }
}
