pub trait HasReadonly {
    type Readonly;

    fn readonly(&self) -> Self::Readonly;
}

pub trait Readonly {
    type Type;

    fn type_(&self) -> Self::Type;
}
