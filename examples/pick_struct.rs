use utility_macros::Pick;

fn main() {}

#[derive(Pick)]
#[utility_macros("Contact" => "email" | "phone_number")]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub phone_number: Option<String>,
}
