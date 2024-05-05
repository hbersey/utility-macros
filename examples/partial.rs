use utility_macros::{HasPartial, Partial};

pub fn main() {
    let full = X::new();
    let partial = full.partial();
    let also_partial = XPartial::from(full.clone());
    let also_full = partial.full().expect("Failed to convert to full");

    assert_eq!(partial, full);
    assert_eq!(partial, also_partial);
    assert_eq!(full, also_full);
}

#[derive(Partial, Debug, Clone, PartialEq)]
pub struct X {
    pub a: String,
    pub b: Option<String>,
    c: String,
    d: Option<String>,
}

impl X {
    pub fn new() -> Self {
        Self {
            a: "a".to_string(),
            b: Some("b".to_string()),
            c: "c".to_string(),
            d: Some("d".to_string()),
        }
    }
}
