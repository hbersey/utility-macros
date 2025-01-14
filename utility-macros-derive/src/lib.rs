mod utils;

mod partial;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn partial(attr: TokenStream, item: TokenStream) -> TokenStream {
    match partial::partial_inner(attr, item) {
        Ok(ts) => ts,
        Err(ts) => ts,
    }
}
