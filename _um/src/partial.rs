use crate::error::Result;

pub trait HasPartial {
    type Partial;

    fn partial(&self) -> Self::Partial;
}

pub trait Partial {
    type Full;

    fn full(&self) -> Result<Self::Full>;
}
