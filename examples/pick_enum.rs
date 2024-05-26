use utility_macros::Pick;

fn main() {}

#[derive(Pick)]
#[utility_macros("CFamily" => "C" | "Cpp" | "ObjectiveC")]
pub enum Languages {
    C { version: u32 },
    Cpp(u32),
    ObjectiveC,
    JavaScript,
    Python,
    Rust,
}
