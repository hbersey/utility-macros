use utility_macros::union;

fn main() {}

union! {
    #[derive(Clone)]
    type MyUnion = A | B | C;
}

#[derive(Clone)]
pub struct A {
    pub a: i32,
    pub b: i32,
}

#[derive(Clone)]
pub struct B {
    pub c: String,
    pub d: String,
}

#[derive(Clone)]
pub struct C {
    pub e: f32,
    pub f: f32,
}
