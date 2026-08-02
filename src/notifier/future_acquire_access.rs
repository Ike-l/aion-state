// use crate::prelude::{AccessFilter, Notifier, RegistryNotifiedAcquireAccess};

use std::task::Poll;

use crate::prelude::{AccessFilter, Notifier, RegistryNotifiedAcquireAccess, Waiter};

pub struct FutureAcquireAccess<
    Notifyee: Notifier, 
    Filter: AccessFilter<Error = Notifyee::Error>,
    Id, IdPassword, ResourceId, Access, Password
> {
    notifyee: crate::prelude::sync::Arc<Notifyee>,
    input: RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
    filter: Filter,
    waiter: crate::prelude::sync::Arc<crate::prelude::sync::Mutex<Waiter>>,
}

impl<Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password> FutureAcquireAccess<Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password> 
    where 
        Notifyee: Notifier<AccessInput = RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
        RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone
{
    pub fn new(
        notifyee: crate::prelude::sync::Arc<Notifyee>,
        input: RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
        filter: Filter,
    ) -> Self {
        let waiter = notifyee.register(input.clone());
        Self { notifyee, input, filter, waiter }
    }
}

impl<Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password> Future for FutureAcquireAccess<Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password> 
    where 
        Notifyee: Notifier<AccessInput = RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
        RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone
{
    type Output = Result<Notifyee::Output, Notifyee::Error>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut waiter = self.waiter.lock();
        waiter.set_waker(cx.waker().clone());

        if waiter.is_ready_to_retry() {
            let result = self.notifyee.acquire_access(self.input.clone());
            match result {
                Ok(result) => {
                    return Poll::Ready(Ok(result))
                },
                Err(error) => {
                    if self.filter.retry(&error) {
                        waiter.set_waiting_to_retry();
                    } else {
                        return Poll::Ready(Err(error))
                    }
                }
            }
        }

        Poll::Pending
    }
}

// impl<'a, T> Drop for FutureResolve<'a, T> {
//     fn drop(&mut self) {
//         // let span = span!(FUNCTION_LEVEL, "FutureResolve Drop");
//         // let _enter = span.enter();

//         // let mut future_resources = self.program_registry.future_resources.lock();

//         // for key in self.cached_keys.iter() {
//         //     let span = span!(FUNCTION_LEVEL, "For key", key =? key);
//         //     let _enter = span.enter();
            
//         //     if let Some(waiters) = future_resources.get_mut(key) {
//         //         event!(FUNCTION_LEVEL, waiters_len =? waiters.len(), "Waiters len before");

//         //         waiters.retain(|w| !Arc::ptr_eq(w, &self.waker_ready));
                
//         //         event!(FUNCTION_LEVEL, waiters_len =? waiters.len(), "Waiters len after");
//         //     }
//         // }
//     }
// }