use crate::prelude::Waiter;

pub mod future_acquire_access;
pub mod access_filter;
pub mod notify_queue;
pub mod waiter;

pub trait Notifier {
    type AccessInput;
    type Error;
    type Output;

    fn register(&self, input: Self::AccessInput) -> crate::prelude::sync::Arc<crate::prelude::sync::Mutex<Waiter>>;
    fn acquire_access(&self, input: Self::AccessInput) -> Result<Self::Output, Self::Error>;
}