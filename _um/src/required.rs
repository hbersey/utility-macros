use crate::error::Result;

pub trait HasRequired {
    type Required;

    fn required(&self) -> Result<Self::Required>;
}

pub trait Required {
    type Type;

    fn type_(&self) -> Self::Type;
}
