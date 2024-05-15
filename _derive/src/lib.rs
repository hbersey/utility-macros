use _um::{
    derive::{
        partial::partial_impl, pick::pick_impl, readonly::readonly_impl, record::record_impl,
        required::required_impl,
    },
    union::union_impl::union_impl,
};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

// Derives the `Partial` trait for a struct.
#[proc_macro_derive(Partial, attributes(partial))]
pub fn derive_partial(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    partial_impl(input).into()
}

/// Derives the `Required` trait for a struct.
#[proc_macro_derive(Required, attributes(required))]
pub fn derive_required(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    required_impl(input).into()
}

/// Derives the `Readonly` trait for a struct.
#[proc_macro_derive(Readonly, attributes(readonly))]
pub fn derive_readonly(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    readonly_impl(input).into()
}

/// Derives the `Record` trait for a struct.
#[proc_macro_derive(Record, attributes(record))]
pub fn derive_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    record_impl(input).into()
}

#[proc_macro_derive(Pick, attributes(pick))]
pub fn derive_pick(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    pick_impl(input).into()
}

/// Create a string union inside me.
#[proc_macro]
pub fn union(item: TokenStream) -> TokenStream {
    union_impl(item.into()).into()
}
