use crate::prelude::{AccessorResult, Releaser, ReleasingResult, sync::Arc};

pub trait AsyncReleaser<'a, Value>: Releaser<'a, Value> {
    fn async_acquire_released_access<AccessResult: AccessorResult<'a, Value>>(
        self: &'a Arc<Self>, 
        input: Self::AccessInput
    ) -> 
        impl Future<Output = Result<ReleasingResult<Value, AccessResult, Self>, Self::Error>> + 'a;
}