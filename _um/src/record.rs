use std::ops::{Index, IndexMut};

pub trait Record: Index<Self::Keys> + IndexMut<Self::Keys> {
    type Keys;
    type Type;

    fn keys(&self) -> Vec<Self::Keys>;

    fn values(&self) -> Vec<&Self::Type>;
    fn values_mut(&mut self) -> Vec<&mut Self::Type>;

    fn entries(&self) -> Vec<(Self::Keys, &Self::Type)>;
    fn entires_mut(&mut self) -> Vec<(Self::Keys, &mut Self::Type)>;
}
