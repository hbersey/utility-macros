use utility_macros::{HasPartial as _, Partial};

pub fn main() {
    let full = X::new();
    let partial = full.partial();
    let also_partial = Y::from(full.clone());
    let also_full = partial.full().expect("Failed to convert to full");

    assert_eq!(full, partial);
    assert_eq!(partial, also_partial);
    assert_eq!(full, also_full);
}

#[derive(Partial, PartialEq, Clone, Debug)]
#[partial(name = Y, derive(PartialEq, Clone, Debug))]
pub struct X {
    #[partial(name = aa)]
    pub a: String,
    pub b: Option<String>,
    c: String,
    d: Option<String>,
    #[partial(skip)]
    pub z: Option<String>,
}

impl X {
    pub fn new() -> Self {
        Self {
            a: "a".to_string(),
            b: Some("b".to_string()),
            c: "c".to_string(),
            d: Some("d".to_string()),
            z: Some("z".to_string()),
        }
    }
}
