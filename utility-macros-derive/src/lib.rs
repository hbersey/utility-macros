mod utils;

mod partial;
mod required;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn partial(attr: TokenStream, item: TokenStream) -> TokenStream {
    match partial::partial_inner(attr, item) {
        Ok(ts) => ts,
        Err(ts) => ts,
    }
}

#[proc_macro_attribute]
pub fn required(attr: TokenStream, item: TokenStream) -> TokenStream {
    match required::required_inner(attr, item) {
        Ok(ts) => ts,
        Err(ts) => ts,
    }
}
