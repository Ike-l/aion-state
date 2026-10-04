use aion_state::prelude::AccessorResult;

#[derive(Debug, PartialEq)]
pub enum AccessResult<T: Transmutable> {
    Shared(T),
    Unique(T),
    Owned(T),
}

pub trait Transmutable {
    fn transmute(self) -> Self;
}

impl<T: Transmutable> AccessorResult<T> for AccessResult<T> {
    fn to_shared(value: T) -> Self {
        let shared = value.transmute();
        AccessResult::Shared(shared)
    }

    fn new_unique(value: T) -> Self {
        AccessResult::Unique(value)
    }

    fn to_owned(value: T) -> Self {
        AccessResult::Owned(value)
    }
}