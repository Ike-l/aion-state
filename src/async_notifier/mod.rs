use crate::prelude::{AccessorResult, Notifier};

pub mod async_future_acquire_access;

pub trait AsyncNotifier<'a, Value>: Notifier<'a, Value> {
    fn async_acquire_access<AccessResult: AccessorResult<Value>>(&'a self, input: Self::AccessInput) -> impl Future<Output = Result<AccessResult, Self::Error>> + 'a;
}