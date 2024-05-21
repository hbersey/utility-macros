use utility_macros::{HasPartial as _, Partial};

pub fn main() {
    let full = X::new();
    let partial = full.partial();
    let also_partial = Y::from(full.clone());
    let also_full = partial.type_().expect("Failed to convert to full");

    assert_eq!(full, partial);
    assert_eq!(partial, also_partial);
    assert_eq!(full, also_full);
}

#[derive(Partial, PartialEq, Clone, Debug)]
#[utility_macros(name = "Y", derive = "Partial, PartialEq, Clone, Debug")]
pub struct X {
    #[utility_macros(name = "aa")]
    pub a: String,
    pub b: Option<String>,
    c: String,
    d: Option<String>,
    #[utility_macros(skip)]
    pub z: Option<String>,
}

impl X {
    pub fn new() -> Self {
        Self {
            a: "a".to_string(),
            b: Some("b".to_string()),
            c: "c".to_string(),
            d: Some("d".to_string()),
            z: None,
        }
    }
}
