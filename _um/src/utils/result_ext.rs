use std::fmt::Debug;

pub trait ResultExt<T, E> {
    fn or_panic(self) -> T;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: Debug,
{
    fn or_panic(self) -> T {
        match self {
            Ok(value) => value,
            Err(e) => panic!("{:?}", e),
        }
    }
}
