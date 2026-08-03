use crate::prelude::{AccessorResult, Waiter, sync::{Arc, Mutex}};

pub mod async_future_acquire_access;

pub trait AsyncNotifier<Value> {
    type AccessInput;
    type Error;

    fn register_waiter(&self, input: Self::AccessInput) -> Arc<Mutex<Waiter>>;
    fn unregister_waiter(&self, input: &Self::AccessInput, waiter: &Arc<Mutex<Waiter>>);
    fn async_acquire_access<'a, AccessResult: AccessorResult<'a, Value>>(&'a self, input: Self::AccessInput) -> impl Future<Output = Result<AccessResult, Self::Error>> + 'a;
}