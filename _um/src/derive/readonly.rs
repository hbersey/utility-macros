use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field};

use crate::{
    derive::{
        container_attributes::{container_attributes, ContainerAttributesData},
        field_attributes::{field_attributes, FieldAttributesContext, FieldAttributesData},
    },
    option::is_option,
};

pub fn readonly_impl(
    DeriveInput {
        attrs,
        ident: type_ident,
        data,
        ..
    }: DeriveInput,
) -> TokenStream {
    let ContainerAttributesData {
        ident: readonly_ident,
        derives,
        rename_all,
    } = container_attributes("readonly", attrs, format_ident!("Readonly{}", type_ident));

    let field_attr_context = FieldAttributesContext {
        helper: "readonly",
        rename_all,
    };

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
            ident: type_ident,
            ..
        } = field;

        let type_ident = type_ident.clone().expect("Expected ident");

        let FieldAttributesData {
            ident: readonly_ident,
            skip,
        } = field_attributes(&field_attr_context, field);

        if skip {
            if is_option(ty) {
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
            #readonly_ident: #ty,
        });

        impl_new_params.push(quote! {
            #readonly_ident: #ty
        });

        impl_new_body.push(quote! {
            #readonly_ident
        });

        impl_getters.push(quote! {
            pub fn #readonly_ident(&self) -> &#ty {
                &self.#type_ident
            }
        });

        to_readonly_body.push(quote! {
            #readonly_ident: self.#type_ident.clone(),
        });

        to_type_body.push(quote! {
            #type_ident: self.#readonly_ident.clone(),
        });

        partial_eq_conditions.push(quote! {
            self.#type_ident == other.#readonly_ident
        });
    }

    let derives = if derives.is_empty() {
        quote! {
            #[derive(Clone, Debug, PartialEq)]
        }
    } else {
        quote! {
            #[derive(#(#derives),*)]
        }
    };

    let partial_eq_impl = if impl_partial_eq {
        quote! {
            impl PartialEq<#readonly_ident> for #type_ident {
                fn eq(&self, other: &#readonly_ident) -> bool {
                    #(#partial_eq_conditions)&& *
                }
            }
        }
    } else {
        quote! {
            impl PartialEq<#readonly_ident> for #type_ident {
                fn eq(&self, _: &#readonly_ident) -> bool {
                    panic!("Partial equality can't be implemented for types that have skipped, non-optional fields");
                }
            }
        }
    };

    quote! {
        #(#static_assertions)*

        #derives
        pub struct #readonly_ident {
            #(#struct_body)*
        }

        impl #readonly_ident {
            pub fn new(#(#impl_new_params),*) -> Self {
                Self {
                    #(#impl_new_body),*
                }
            }

            #(#impl_getters)*
        }

        impl ::utility_macros::_um::readonly::HasReadonly for #type_ident {
            type Readonly = #readonly_ident;

            fn readonly(&self) -> Self::Readonly {
                Self::Readonly {
                    #(#to_readonly_body)*
                }
            }
        }

        impl ::utility_macros::_um::readonly::Readonly for #readonly_ident {
            type Type = #type_ident;

            fn type_(&self) -> Self::Type {
                #type_ident {
                    #(#to_type_body)*
                }
            }
        }

        impl From<#type_ident> for #readonly_ident {
            fn from(value: #type_ident) -> Self {
                ::utility_macros::_um::readonly::HasReadonly::readonly(&value)
            }
        }

        impl From<#readonly_ident> for #type_ident {
            fn from(value: #readonly_ident) -> Self {
                ::utility_macros::_um::readonly::Readonly::type_(&value)
            }
        }

        #partial_eq_impl
    }
}
