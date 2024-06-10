use utility_macros::{HasReadonly as _, Readonly};

fn main() {
    let a = A { x: 1, y: 2, z: 3 };
    let b = B::new(1, 2, 3);

    assert_eq!(a, b);
    assert_eq!(a, b.type_());
    assert_eq!(a.readonly(), b);
}

#[derive(Readonly, Debug, PartialEq, Clone)]
#[utility_macros(name = "B", derive "Debug, PartialEq, Clone")]
pub struct A {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
