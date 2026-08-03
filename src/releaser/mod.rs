use crate::prelude::{AccessorResult, ReleasingResult, sync::Arc};

pub mod releasing_result;

pub trait Releaser<Value> {
    type AccessError;
    type AccessInput;

    type ReleaseInput;

    fn acquire_access<'a, AccessResult: AccessorResult<'a, Value>>(self: &'a Arc<Self>, access_input: Self::AccessInput) -> Result<ReleasingResult<Value, AccessResult, Self>, Self::AccessError>;
    fn release_access(&self, release_input: &Self::ReleaseInput);
}