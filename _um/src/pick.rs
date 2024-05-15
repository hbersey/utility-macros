pub trait HasPick {
    type Pick: Pick;

    fn pick(&self) -> Self::Pick;
}

pub trait Pick {
    type Type;
}
