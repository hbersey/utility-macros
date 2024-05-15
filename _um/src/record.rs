use crate::error::Result;
use std::ops::{Index, IndexMut};

pub trait HasRecord: Sized {
    type Record;

    fn as_str(&self) -> &'static str;
    fn try_from_str(s: &str) -> Result<Self>;
}

pub trait Record: Index<Self::Keys> + IndexMut<Self::Keys> + Sized {
    type Keys: HasRecord<Record = Self>;
    type Type;

    const N: usize;

    fn keys() -> Vec<Self::Keys>;

    fn values(&self) -> Vec<&Self::Type>;
    fn values_mut(&mut self) -> Vec<&mut Self::Type>;

    fn entries(&self) -> Vec<(Self::Keys, &Self::Type)>;
    fn entires_mut(&mut self) -> Vec<(Self::Keys, &mut Self::Type)>;

    fn try_from_entries(entries: Vec<(Self::Keys, Self::Type)>) -> Result<Self>;
}
