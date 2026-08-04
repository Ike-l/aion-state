use aion_state::prelude::AccessorResult;

#[derive(Debug, PartialEq)]
pub enum AccessResult<'a, T> {
    Shared(&'a T),
    Unique(&'a mut T),
    Owned(T),
}

impl<'a, T> AccessorResult<'a, T> for AccessResult<'a, T> {
    fn new_shared(value: &'a T) -> Self {
        AccessResult::Shared(value)
    }

    fn new_unique(value: &'a mut T) -> Self {
        AccessResult::Unique(value)
    }

    fn new_owned(value: T) -> Self {
        AccessResult::Owned(value)
    }
}