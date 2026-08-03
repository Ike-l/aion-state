use crate::prelude::{AccessorResult, Notifier};

pub mod async_future_acquire_access;

pub trait AsyncNotifier<Value>: Notifier<Value> {
    fn async_acquire_access<'a, AccessResult: AccessorResult<'a, Value>>(&'a self, input: Self::AccessInput) -> impl Future<Output = Result<AccessResult, Self::Error>> + 'a;
}