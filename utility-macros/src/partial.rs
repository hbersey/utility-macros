use crate::Result;

pub trait Partial {
    type Full;

    fn try_as_full(self) -> Result<Self::Full>;
}

pub trait HasPartial {
    type Partial;

    fn as_partial(self) -> Self::Partial;
}
