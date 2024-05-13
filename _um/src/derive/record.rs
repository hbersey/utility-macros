use convert_case::{Case, Casing as _};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use syn::{Data, DeriveInput, Ident, Meta};

pub fn record_impl(
    DeriveInput {
        attrs: type_attrs,
        ident: type_ident,
        data,
        ..
    }: DeriveInput,
) -> TokenStream {
    let Data::Enum(data) = data else {
        panic!("Expected Enum")
    };

    let mut impls = Vec::new();

    for attr in &type_attrs {
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

        let mut tokens = meta.tokens.clone().into_iter().peekable();

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

        let mut derive = None;
        match tokens.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {
                tokens.next();

                let Some(TokenTree::Ident(ident)) = tokens.next() else {
                    panic!("Expected ident");
                };

                if ident != "derive" {
                    panic!("Expected \"derive\"");
                }

                let Some(TokenTree::Group(group)) = tokens.next() else {
                    panic!("Expected group");
                };

                derive = Some(group.stream());
            },
            _ => {}
        }

        let derive = derive.map(|ts| quote!{
            #[derive(#ts)]
        }).unwrap_or_else(|| quote! {
            #[derive(Clone, Debug, PartialEq)]
        });

        let variants: Vec<syn::Variant> = data.variants.clone().into_iter().collect::<Vec<_>>();

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

            #derive
            pub struct #ident {
                #(pub #idents: #ty),*
            }

            impl utility_macros::_um::record::HasRecord for #type_ident {
                type Record = #ident;
            }

            impl utility_macros::_um::record::Record for #ident {
                type Keys = #type_ident;
                type Type = #ty;

                fn keys(&self) -> Vec<Self::Keys> {
                    vec![
                        #(#type_ident::#variants),*
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
                        #((#type_ident::#variants, &self.#idents)),*
                    ]
                }
                fn entires_mut(&mut self) -> Vec<(Self::Keys, &mut Self::Type)> {
                    vec![
                        #((#type_ident::#variants, &mut self.#idents)),*
                    ]
                }

                fn try_from_entries(entries: Vec<(Self::Keys, Self::Type)>) -> utility_macros::_um::error::Result<Self> {
                    #(let mut #idents = Option::<Self::Type>::None;)*

                    for (k, v) in entries {
                        match k {
                            #(
                                #type_ident::#variants => {
                                    if let Some(_) = #idents {
                                        return Err(utility_macros::_um::error::Error::DuplicateKey(
                                            concat!(stringify!(#type_ident), "::", stringify!(#variants))
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
                                concat!(stringify!(#type_ident), "::", stringify!(#variants))
                            ))?
                        ),*
                    })
                }
            }

            impl std::ops::Index<#type_ident> for #ident {
                type Output = #ty;

                fn index(&self, index: #type_ident) -> &Self::Output {
                    match index {
                        #(#type_ident::#variants => &self.#idents),*
                    }
                }
            }

            impl std::ops::IndexMut<#type_ident> for #ident {
                fn index_mut(&mut self, index: #type_ident) -> &mut Self::Output {
                    match index {
                        #(#type_ident::#variants => &mut self.#idents),*
                    }
                }

            }
        });
    }

    quote! {
        #(#impls)*
    }
}
