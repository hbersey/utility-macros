use utility_macros::union;

fn main() {}

union! {
    type Side = "top" | "bottom";
    #[derive(Clone)]
    type Axis = "x" | "y";
}
