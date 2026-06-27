use crate::prelude::{DeaccessingResult, sync::Arc};

pub mod deaccessing_result;

pub trait Deaccessor {
    type AccessResult<'a> where Self: 'a;
    type AccessError;
    type AccessInput;

    type ReleaseInput;

    fn acquire_access(self: &Arc<Self>, access_input: Self::AccessInput) -> Result<DeaccessingResult<Self::AccessResult<'_>, Self>, Self::AccessError>;
    fn release_access(&self, release_input: &Self::ReleaseInput);
}