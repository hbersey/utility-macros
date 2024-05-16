use crate::error::Result;

pub trait HasPick {
    type Pick;

    fn pick(&self) -> Result<Self::Pick>;
}

pub trait Pick {
    type Type;
}
