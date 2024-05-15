use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Meta};

use crate::expect_token::{expect_token, peek_token};

fn impl_struct(
    DataStruct {
        fields: type_fields,
        ..
    }: DataStruct,
    DeriveInput {
        attrs: type_attrs,
        ident: type_ident,
        ..
    }: DeriveInput,
) -> TokenStream {
    let mut impls = Vec::new();
    for attr in &type_attrs {
        let Meta::List(meta) = &attr.meta else {
            continue;
        };

        if meta
            .path
            .segments
            .first()
            .map_or(true, |s| s.ident != "pick")
        {
            continue;
        }

        let mut tokens = meta.tokens.clone().into_iter().peekable();

        let ident = expect_token!(tokens, ident);
        expect_token!(tokens, =>);

        let mut fields = Vec::new();
        let mut types = Vec::new();
        let mut derives = Vec::new();

        loop {
            let field = expect_token!(tokens, ident);

            let Some(ty) = type_fields
                .iter()
                .find(|f| {
                    f.ident
                        .as_ref()
                        .map_or(false, |i| i.to_string() == field.to_string())
                })
                .map(|f| f.ty.clone())
            else {
                panic!("Field `{:?}` not found", field);
            };

            fields.push(field);
            types.push(ty);

            if peek_token!(tokens, punct = ',').is_some() {
                tokens.next();

                expect_token!(tokens, ident = "derive");
                let group = expect_token!(tokens, group, delimiter = Delimiter::Parenthesis);

                let mut group_tokens = group.stream().into_iter().peekable();

                loop {
                    let derive = expect_token!(group_tokens, ident);

                    derives.push(derive);

                    if group_tokens.peek().is_none() {
                        break;
                    }

                    expect_token!(group_tokens, punct = ',');
                }

                break;
            } else if tokens.peek().is_none() {
                break;
            }

            expect_token!(tokens, punct = '|');
        }

        let derives_impl = if derives.is_empty() {
            quote! {
                #[derive(PartialEq, Clone, Debug)]
            }
        } else {
            quote! {
                #[derive(#(#derives),*)]
            }
        };

        impls.push(quote! {
            #derives_impl
            pub struct #ident {
                #(pub #fields: #types),*
            }

            impl ::utility_macros::_um::pick::HasPick for #type_ident {
                type Pick = #ident;

                fn pick(&self) -> Self::Pick {
                    #ident {
                        #(#fields: self.#fields.clone()),*
                    }
                }
            }

            impl ::utility_macros::_um::pick::Pick for #ident {
                type Type = #type_ident;
            }
        });
    }

    quote! {
        #(#impls)*
    }
}

fn impl_enum(data: DataEnum, input: DeriveInput) -> TokenStream {
    quote! {}
}

pub fn pick_impl(input: DeriveInput) -> TokenStream {
    match (&input).data.clone() {
        Data::Struct(data) => impl_struct(data, input),
        Data::Enum(data) => impl_enum(data, input),
        _ => panic!("Expected struct or enum"),
    }
}
