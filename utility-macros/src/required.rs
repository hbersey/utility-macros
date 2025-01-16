use crate::Result;

pub trait Required {
    type NotRequired;

    fn as_not_required(self) -> Self::NotRequired;
}

pub trait HasRequired {
    type Required;

    fn try_as_required(self) -> Result<Self::Required>;
}
