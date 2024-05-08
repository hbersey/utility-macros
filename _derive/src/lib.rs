use _um::{container_attributes::*, field_attributes::*, option::*};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Field};

#[proc_macro_derive(Partial, attributes(partial))]
pub fn derive_partial(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        data,
        ident: full_ident,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let ContainerAttributesData {
        ident: partial_ident,
        derives,
    } = container_attributes("partial", attrs, format_ident!("Partial{}", full_ident));

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
            ident: full_ident,
            ..
        } = field;

        let full_ident = full_ident.clone().expect("Expected ident");

        let FieldAttributesData {
            ident: partial_ident,
            skip,
        } = field_attributes("partial", field);

        if skip {
            if is_option(ty) {
                to_full_body.push(quote! {
                    #full_ident: None,
                });
                partial_eq.push(quote! {
                    self.#full_ident.is_none()
                });
            } else {
                static_assertions.push(quote! {
                    ::utility_macros::_um::_sa::assert_impl_all!(#ty: Default);
                });
                to_full_body.push(quote! {
                    #full_ident: Default::default(),
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
                #partial_ident: self.#full_ident.clone(),
            });
            to_full_body.push(quote! {
                #full_ident: self.#partial_ident.clone(),
            });
            partial_eq.push(quote! {
                self.#full_ident == other.#partial_ident
            });
        } else {
            to_partial_body.push(quote! {
                #partial_ident: Some(self.#full_ident.clone()),
            });
            to_full_body.push(quote! {
                #full_ident: self.#partial_ident.clone().ok_or_else(|| ::utility_macros::_um::error::Error::MissingField(stringify!(#partial_ident)))?,
            });
            partial_eq.push(quote! {
                other.#partial_ident.clone().map_or(false, |val| self.#full_ident == val)
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
            impl PartialEq<#partial_ident> for #full_ident {
                fn eq(&self, other: &#partial_ident) -> bool {
                    #(#partial_eq)&& *
                }
            }
        }
    } else {
        quote! {
            impl PartialEq<#partial_ident> for #full_ident {
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

        impl ::utility_macros::_um::partial::HasPartial for #full_ident {
            type Partial = #partial_ident;

            fn partial(&self) -> Self::Partial {
                Self::Partial {
                    #(#to_partial_body)*
                }
            }
        }

        impl ::utility_macros::_um::partial::Partial for #partial_ident {
            type Full = #full_ident;

            fn full(&self) -> ::utility_macros::_um::error::Result<Self::Full> {
                Ok(#full_ident {
                    #(#to_full_body)*
                })
            }
        }

        impl From<#full_ident> for #partial_ident {
            fn from(full: #full_ident) -> Self {
                ::utility_macros::_um::partial::HasPartial::partial(&full)
            }
        }

        impl TryFrom<#partial_ident> for #full_ident {
            type Error = ::utility_macros::_um::error::Error;

            fn try_from(partial: #partial_ident) -> ::utility_macros::_um::error::Result<Self> {
                ::utility_macros::_um::partial::Partial::full(&partial)
            }
        }

       #partial_eq_impl
    }
    .into()
}
