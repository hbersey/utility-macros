use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field};

use crate::{
    derive::{
        container_attributes::{container_attributes, ContainerAttributesData},
        field_attributes::{field_attributes, FieldAttributesContext, FieldAttributesData},
    },
    option::{as_required, is_option},
};

pub fn required_impl(
    DeriveInput {
        attrs,
        ident: type_ident,
        data,
        ..
    }: DeriveInput,
) -> TokenStream {
    let ContainerAttributesData {
        ident: required_ident,
        derives,
        rename_all,
    } = container_attributes("required", attrs, format_ident!("Required{}", type_ident));

    let field_attr_context = FieldAttributesContext {
        helper: "required",
        rename_all,
    };

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
            ident: full_ident,
            ..
        } = field;

        let type_ident = full_ident.clone().expect("Expected ident");

        let FieldAttributesData {
            ident: required_ident,
            skip,
        } = field_attributes(&field_attr_context, field);

        if skip {
            if is_option(ty) {
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

        let required_ty = as_required(ty);
        struct_body.push(quote! {
            #vis #required_ident: #required_ty,
        });

        static_assertions.push(quote! {
            ::utility_macros::_um::_sa::assert_impl_all!(#required_ty: Clone);
        });

        if is_option(ty) {
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

    let derives = if derives.is_empty() {
        quote! {}
    } else {
        quote! {
            #[derive(#(#derives),*)]
        }
    };

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

        #derives
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
