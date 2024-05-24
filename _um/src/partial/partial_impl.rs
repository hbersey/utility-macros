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

pub fn partial_impl(
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
    let mut to_partial_body = Vec::new();
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
        let partial_ident = attrs.field_ident(type_ident.clone());
        let field_attrs = parse_field_variant_attributes(field.attrs.clone()).or_panic();
        let partial_ident = field_attrs.ident(partial_ident);

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

        let opt_ty = ty.as_option();
        struct_body.push(quote! {
            #vis #partial_ident: #opt_ty,
        });

        static_assertions.push(quote! {
            ::utility_macros::_um::_sa::assert_impl_all!(#opt_ty: Clone);
        });

        if ty.is_option() {
            to_partial_body.push(quote! {
                #partial_ident: self.#type_ident.clone(),
            });
            to_type_body.push(quote! {
                #type_ident: self.#partial_ident.clone(),
            });
            partial_eq.push(quote! {
                self.#type_ident == other.#partial_ident
            });
        } else {
            to_partial_body.push(quote! {
                #partial_ident: Some(self.#type_ident.clone()),
            });
            to_type_body.push(quote! {
                #type_ident: self.#partial_ident.clone().ok_or_else(|| ::utility_macros::_um::error::Error::MissingField(stringify!(#partial_ident)))?,
            });
            partial_eq.push(quote! {
                other.#partial_ident.clone().map_or(false, |val| self.#type_ident == val)
            });
        }
    }

    let partial_ident = attrs.ident(format_ident!("Partial{}", type_ident));

    let derive_statement = attrs
        .derive_statement("PartialEq, Clone, Debug, Default")
        .or_panic();

    let partial_eq_impl = if impl_partial_eq {
        quote! {
            impl PartialEq<#partial_ident> for #type_ident {
                fn eq(&self, other: &#partial_ident) -> bool {
                    #(#partial_eq)&& *
                }
            }
        }
    } else {
        quote! {
            impl PartialEq<#partial_ident> for #type_ident {
                fn eq(&self, _: &#partial_ident) -> bool {
                    panic!("Partial equality can't be implemented for types that have skipped, non-optional fields");
                }
            }
        }
    };

    quote! {
        #(#static_assertions)*

        #derive_statement
        pub struct #partial_ident {
            #(#struct_body)*
        }

        impl ::utility_macros::_um::partial::HasPartial for #type_ident {
            type Partial = #partial_ident;

            fn partial(&self) -> Self::Partial {
                Self::Partial {
                    #(#to_partial_body)*
                }
            }
        }

        impl ::utility_macros::_um::partial::Partial for #partial_ident {
            type Type = #type_ident;

            fn type_(&self) -> ::utility_macros::_um::error::Result<Self::Type> {
                Ok(#type_ident {
                    #(#to_type_body)*
                })
            }
        }

        impl From<#type_ident> for #partial_ident {
            fn from(type_: #type_ident) -> Self {
                ::utility_macros::_um::partial::HasPartial::partial(&type_)
            }
        }

        impl TryFrom<#partial_ident> for #type_ident {
            type Error = ::utility_macros::_um::error::Error;

            fn try_from(partial: #partial_ident) -> ::utility_macros::_um::error::Result<Self> {
                ::utility_macros::_um::partial::Partial::type_(&partial)
            }
        }
       #partial_eq_impl
    }
}
