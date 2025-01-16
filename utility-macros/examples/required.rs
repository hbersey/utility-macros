use utility_macros::required;

#[required(RequiredProps)]
pub struct Props<T> {
    a: u8,
    pub b: Option<T>,
}

fn main() {}
