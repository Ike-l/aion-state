use crate::prelude::{AccessorResult, Releaser, ReleasingResult, ViewReleaser, sync::Arc};

pub trait AsyncReleaser<'a, Value>: Releaser<'a, Value> {
    fn async_acquire_released_access<AccessResult: AccessorResult<Value>>(
        self: &'a Arc<Self>, 
        input: Self::AccessInput
    ) -> 
        impl Future<Output = Result<ReleasingResult<'a, Value, AccessResult, Self>, Self::Error>> + 'a;
}

pub trait AsyncViewReleaser<'a, View>: ViewReleaser<'a, View> {
    fn async_acquire_released_view<AccessResult: AccessorResult<View>>(
        self: &'a Arc<Self>,
        access_input: Self::AccessInput,
    ) -> impl Future<Output = Result<ReleasingResult<'a, View, AccessResult, Self>, Self::Error>> + 'a;
}
