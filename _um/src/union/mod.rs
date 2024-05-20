mod static_str_union;
pub use static_str_union::StaticStrUnion;

mod union;
pub use union::Union;

mod union_impl;
pub use union_impl::union_impl as derive;
