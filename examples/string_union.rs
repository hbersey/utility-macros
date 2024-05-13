use utility_macros::{string_union, StringUnion};

fn main() {
    let a = MyStringUnion::Foo;
    let b = MyStringUnion::try_from_str("foo").unwrap();
    assert_eq!(a, b);
    assert_eq!(a, "foo");
}

string_union! {
    type MyStringUnion = "foo" | "bar" | "baz";
}
