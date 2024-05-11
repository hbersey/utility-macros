use utility_macros::Record;

fn main() {}

#[derive(Record)]
#[record(FruitId => u32)]
pub enum Fruit {
    Apple,
    Banana,
    Kiwi,
    Strawberry,
}
