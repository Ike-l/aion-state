use crate::prelude::{AccessorResult, ReleasingResult};

pub mod releasing_result;

pub trait Releaser<Value> {
    type AccessError;
    type AccessInput;

    type ReleaseInput;

    // because the import is from prelude
    #[allow(clippy::disallowed_types)]
    fn acquire_access<'a, AccessResult: AccessorResult<'a, Value>>(self: &'a crate::prelude::sync::Arc<Self>, access_input: Self::AccessInput) -> Result<ReleasingResult<Value, AccessResult, Self>, Self::AccessError>;
    fn release_access(&self, release_input: &Self::ReleaseInput);
}