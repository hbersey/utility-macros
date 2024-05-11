use _um::{derive_utils::{container_attributes::*, field_attributes::*}, option::*};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenTree};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Field, Meta};

/// Derives the `Partial` trait for a struct.
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
        rename_all,
    } = container_attributes("partial", attrs, format_ident!("Partial{}", full_ident));

    let field_attr_context = FieldAttributesContext {
        helper: "partial",
        rename_all
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
            ident: full_ident,
            ..
        } = field;

        let full_ident = full_ident.clone().expect("Expected ident");

        let FieldAttributesData {
            ident: partial_ident,
            skip,
        } = field_attributes(&field_attr_context, field);

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
            type Type = #full_ident;

            fn type_(&self) -> ::utility_macros::_um::error::Result<Self::Type> {
                Ok(#full_ident {
                    #(#to_full_body)*
                })
            }
        }

        impl From<#full_ident> for #partial_ident {
            fn from(type_: #full_ident) -> Self {
                ::utility_macros::_um::partial::HasPartial::partial(&type_)
            }
        }

        impl TryFrom<#partial_ident> for #full_ident {
            type Error = ::utility_macros::_um::error::Error;

            fn try_from(partial: #partial_ident) -> ::utility_macros::_um::error::Result<Self> {
                ::utility_macros::_um::partial::Partial::type_(&partial)
            }
        }

       #partial_eq_impl
    }
    .into()
}

/// Derives the `Required` trait for a struct.
#[proc_macro_derive(Required, attributes(required))]
pub fn derive_required(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        data,
        ident: type_ident,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let ContainerAttributesData {
        ident: required_ident,
        derives,
        rename_all,
    } = container_attributes("required", attrs, format_ident!("Required{}", type_ident));

    let field_attr_context = FieldAttributesContext {
        helper: "required",
        rename_all
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
    .into()
}

/// Derives the `Readonly` trait for a struct.
#[proc_macro_derive(Readonly, attributes(readonly))]
pub fn derive_readonly(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        data,
        ident: type_ident,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let ContainerAttributesData {
        ident: readonly_ident,
        derives,
        rename_all,
    } = container_attributes("readonly", attrs, format_ident!("Readonly{}", type_ident));

    let field_attr_context = FieldAttributesContext {
        helper: "readonly",
        rename_all
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
        quote! {}
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
    .into()
}

/// Derives the `Record` trait for a struct.
#[proc_macro_derive(Record, attributes(record))]
pub fn derive_record(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        attrs: enum_attrs,
        ident: enum_ident,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let Data::Enum(data) = data else {
        panic!("Expected Enum")
    };

    let mut impls = Vec::new();

    for attr in &enum_attrs {
        let Meta::List(meta) = &attr.meta else {
            continue;
        };

        if meta
            .path
            .segments
            .first()
            .map_or(true, |s| s.ident != "record")
        {
            continue;
        }

        let mut tokens = meta.tokens.clone().into_iter();

        let Some(TokenTree::Ident(ident)) = tokens.next() else {
            panic!("Expected ident");
        };

        match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {}
            _ => {
                panic!("Expected \"=>\"");
            }
        }

        match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {}
            _ => {
                panic!("Expected \"=>\"");
            }
        }

        let ty = tokens.next().expect("Expected type");
        match &ty {
            TokenTree::Literal(_) | TokenTree::Punct(_) => panic!("Expected ident or group"),
            _ => {}
        }

        let variants = data.variants.clone().into_iter().collect::<Vec<_>>();

        let idents = variants
            .clone()
            .into_iter()
            .map(|v| {
                Ident::new(
                    v.ident.to_string().to_case(Case::Snake).as_str(),
                    Span::call_site(),
                )
            })
            .collect::<Vec<_>>();

        impls.push(quote! {
            utility_macros::_um::_sa::assert_impl_all! (#ty: Sized);

            pub struct #ident {
                #(pub #idents: #ty),*
            }

            impl utility_macros::_um::record::HasRecord for #enum_ident {
                type Record = #ident;
            }

            impl utility_macros::_um::record::Record for #ident {
                type Keys = #enum_ident;
                type Type = #ty;

                fn keys(&self) -> Vec<Self::Keys> {
                    vec![
                        #(#enum_ident::#variants),*
                    ]
                }

                fn values(&self) -> Vec<&Self::Type> {
                    vec![
                        #(&self.#idents),*
                    ]
                }
                fn values_mut(&mut self) -> Vec<&mut Self::Type> {
                    vec![
                        #(&mut self.#idents),*
                    ]
                }

                fn entries(&self) -> Vec<(Self::Keys, &Self::Type)> {
                    vec![
                        #((#enum_ident::#variants, &self.#idents)),*
                    ]
                }
                fn entires_mut(&mut self) -> Vec<(Self::Keys, &mut Self::Type)> {
                    vec![
                        #((#enum_ident::#variants, &mut self.#idents)),*
                    ]
                }

                fn try_from_entries(entries: Vec<(Self::Keys, Self::Type)>) -> utility_macros::_um::error::Result<Self> {
                    #(let mut #idents = Option::<Self::Type>::None;)*

                    for (k, v) in entries {
                        match k {
                            #(
                                #enum_ident::#variants => {
                                    if let Some(_) = #idents {
                                        return Err(utility_macros::_um::error::Error::DuplicateKey(
                                            concat!(stringify!(#enum_ident), "::", stringify!(#variants))
                                        ));
                                    }
                                    #idents = Some(v);
                                }
                            ),*
                        }
                    } 

                    Ok(Self {
                        #(
                            #idents: #idents.ok_or(utility_macros::_um::error::Error::MissingKey(
                                concat!(stringify!(#enum_ident), "::", stringify!(#variants))
                            ))?
                        ),*
                    })
                }
            }

            impl std::ops::Index<#enum_ident> for #ident {
                type Output = #ty;

                fn index(&self, index: #enum_ident) -> &Self::Output {
                    match index {
                        #(#enum_ident::#variants => &self.#idents),*
                    }
                }
            }

            impl std::ops::IndexMut<#enum_ident> for #ident {
                fn index_mut(&mut self, index: #enum_ident) -> &mut Self::Output {
                    match index {
                        #(#enum_ident::#variants => &mut self.#idents),*
                    }
                }

            }
        });
    }

    quote! {
        #(#impls)*
    }
    .into()
}
