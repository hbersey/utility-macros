use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field};

use crate::{
    derive::{
        container_attributes::{container_attributes, ContainerAttributesData},
        field_attributes::{field_attributes, FieldAttributesContext, FieldAttributesData},
    },
    option::{as_option, is_option},
};

pub fn partial_impl(
    DeriveInput {
        attrs,
        ident: type_ident,
        data,
        ..
    }: DeriveInput,
) -> TokenStream {
    let ContainerAttributesData {
        ident: partial_ident,
        derives,
        rename_all,
    } = container_attributes("partial", attrs, format_ident!("Partial{}", type_ident));

    let field_attr_context = FieldAttributesContext {
        helper: "partial",
        rename_all,
    };

    let Data::Struct(data) = data else {
        panic!("Expected struct")
    };

    let mut static_assertions = Vec::new();
    let mut struct_body = Vec::new();
    let mut to_partial_body = Vec::new();
    let mut to_full_body = Vec::new();
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

        let FieldAttributesData {
            ident: partial_ident,
            skip,
        } = field_attributes(&field_attr_context, field);

        if skip {
            if is_option(ty) {
                to_full_body.push(quote! {
                    #type_ident: None,
                });
                partial_eq.push(quote! {
                    self.#type_ident.is_none()
                });
            } else {
                static_assertions.push(quote! {
                    ::utility_macros::_um::_sa::assert_impl_all!(#ty: Default);
                });
                to_full_body.push(quote! {
                    #type_ident: Default::default(),
                });
                impl_partial_eq = false;
            }
            continue;
        }

        let opt_ty = as_option(ty);
        struct_body.push(quote! {
            #vis #partial_ident: #opt_ty,
        });

        static_assertions.push(quote! {
            ::utility_macros::_um::_sa::assert_impl_all!(#opt_ty: Clone);
        });

        if is_option(ty) {
            to_partial_body.push(quote! {
                #partial_ident: self.#type_ident.clone(),
            });
            to_full_body.push(quote! {
                #type_ident: self.#partial_ident.clone(),
            });
            partial_eq.push(quote! {
                self.#type_ident == other.#partial_ident
            });
        } else {
            to_partial_body.push(quote! {
                #partial_ident: Some(self.#type_ident.clone()),
            });
            to_full_body.push(quote! {
                #type_ident: self.#partial_ident.clone().ok_or_else(|| ::utility_macros::_um::error::Error::MissingField(stringify!(#partial_ident)))?,
            });
            partial_eq.push(quote! {
                other.#partial_ident.clone().map_or(false, |val| self.#type_ident == val)
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

        #derives
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
                    #(#to_full_body)*
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
