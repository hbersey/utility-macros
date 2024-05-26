use _um::{partial, pick, readonly, record, required, union};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

// Derives the `Partial` trait for a struct.
// TODO remove partial attribute
#[proc_macro_derive(Partial, attributes(utility_macros))]
pub fn derive_partial(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    partial::derive(input).into()
}

/// Derives the `Required` trait for a struct.
#[proc_macro_derive(Required, attributes(utility_macros))]
pub fn derive_required(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    required::derive(input).into()
}

/// Derives the `Readonly` trait for a struct.
#[proc_macro_derive(Readonly, attributes(readonly))]
pub fn derive_readonly(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    readonly::derive(input).into()
}

/// Derives the `Record` trait for a struct.
#[proc_macro_derive(Record, attributes(record))]
pub fn derive_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    record::derive(input).into()
}

#[proc_macro_derive(Pick, attributes(utility_macros))]
pub fn derive_pick(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    pick::derive(input).into()
}

/// Create a string union inside me.
#[proc_macro]
pub fn union(item: TokenStream) -> TokenStream {
    union::derive(item.into()).into()
}
