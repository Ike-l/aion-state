use crate::prelude::{AccessorResult, ReleasingResult, sync::Arc};

pub mod releasing_result;

pub trait Release {
    type ReleaseInput;
    fn release_access(&self, release_input: &Self::ReleaseInput);
}

pub trait Releaser<'a, Value>: Release {
    type Error;
    type AccessInput;

    fn acquire_released_access<AccessResult: AccessorResult<Value>>(self: &'a Arc<Self>, access_input: Self::AccessInput) -> Result<ReleasingResult<'a, Value, AccessResult, Self>, Self::Error>;
}

pub trait ViewReleaser<'a, View>: Release {
    type Error;
    type AccessInput;

    fn acquire_released_view<AccessResult: AccessorResult<View>>(
        self: &'a Arc<Self>,
        access_input: Self::AccessInput,
    ) -> Result<ReleasingResult<'a, View, AccessResult, Self>, Self::Error>;
}
