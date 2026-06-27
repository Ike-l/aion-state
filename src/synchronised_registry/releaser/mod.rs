use crate::prelude::{ReleasingResult, sync::Arc};

pub mod releasing_result;

pub trait Releaser {
    type AccessResult<'a> where Self: 'a;
    type AccessError;
    type AccessInput;

    type ReleaseInput;

    fn acquire_access(self: &Arc<Self>, access_input: Self::AccessInput) -> Result<ReleasingResult<Self::AccessResult<'_>, Self>, Self::AccessError>;
    fn release_access(&self, release_input: &Self::ReleaseInput);
}