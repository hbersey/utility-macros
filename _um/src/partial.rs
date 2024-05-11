use crate::error::Result;

pub trait HasPartial {
    type Partial;

    fn partial(&self) -> Self::Partial;
}

pub trait Partial {
    type Type;

    fn type_(&self) -> Result<Self::Type>;
}
