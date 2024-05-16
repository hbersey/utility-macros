use utility_macros::Pick;

fn main() {}

#[derive(Pick)]
#[pick(CFamily => C | Cpp | ObjectiveC)]
pub enum Languages {
    C,
    Cpp,
    ObjectiveC,
    JavaScript,
    Python,
    Rust,
}
