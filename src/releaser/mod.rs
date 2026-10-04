use crate::prelude::{AccessorResult, ReleasingResult, sync::Arc};

pub mod releasing_result;

pub trait Releaser<'a, Value> {
    type Error;
    type AccessInput;

    type ReleaseInput;

    fn acquire_released_access<AccessResult: AccessorResult<'a, Value>>(self: &'a Arc<Self>, access_input: Self::AccessInput) -> Result<ReleasingResult<Value, AccessResult, Self>, Self::Error>;
    fn release_access(&self, release_input: &Self::ReleaseInput);
}