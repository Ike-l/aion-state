use crate::prelude::{AccessorResult, Waiter};

pub mod future_acquire_access;
pub mod access_filter;
pub mod notify_queue;
pub mod waiter;

pub trait Notifier<Value> {
    type AccessInput;
    type Error;

    fn register_waiter(&self, input: Self::AccessInput) -> crate::prelude::sync::Arc<crate::prelude::sync::Mutex<Waiter>>;
    fn unregister_waiter(&self, input: &Self::AccessInput, waiter: &crate::prelude::sync::Arc<crate::prelude::sync::Mutex<Waiter>>);
    fn acquire_access<'a, AccessResult: AccessorResult<'a, Value>>(&'a self, input: Self::AccessInput) -> Result<AccessResult, Self::Error>;
}