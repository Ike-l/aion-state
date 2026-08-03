use crate::prelude::{AccessorResult, Releaser, ReleasingResult, sync::Arc};

pub trait AsyncReleaser<Value>: Releaser<Value> {
    fn async_acquire_released_access<'a, AccessResult: AccessorResult<'a, Value>>(
        self: &'a Arc<Self>, 
        input: Self::AccessInput
    ) -> 
        impl Future<Output = Result<ReleasingResult<Value, AccessResult, Self>, Self::Error>> + 'a;
}