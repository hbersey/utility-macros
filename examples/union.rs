use utility_macros::union;

fn main() {}

union! {
    type MyUnion = A | B | C;
}

pub struct A {
    pub a: i32,
    pub b: i32,
}
pub struct B {
    pub c: String,
    pub d: String,
}

pub struct C {
    pub e: f32,
    pub f: f32,
}
