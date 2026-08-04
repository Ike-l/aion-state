use std::{marker::PhantomData, task::Poll};

use crate::prelude::{AccessFilter, AccessorResult, Notifier, RegistryOwnedAcquireAccess, Waiter, sync::{Arc, Mutex}};

pub struct FutureAcquireAccess<'a,
    Value,
    Notifyee: Notifier<Value, AccessInput = RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>, 
    Filter: AccessFilter<Error = Notifyee::Error>,
    Id, IdPassword, ResourceId, Access, Password,
    AccessResult
> {
    notifyee: &'a Notifyee,
    input: RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
    filter: Filter,
    waiter: Arc<Mutex<Waiter>>,
    _r: PhantomData<AccessResult>,
    _v: PhantomData<Value>
}

impl<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> FutureAcquireAccess<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: Notifier<Value, AccessInput = RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
        RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone
{
    pub fn new(
        notifyee: &'a Notifyee,
        input: RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
        filter: Filter,
    ) -> Self {
        let waiter = notifyee.register_waiter(input.clone());
        Self { notifyee, input, filter, waiter, _r: Default::default(), _v: Default::default() }
    }
}

impl<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Future for FutureAcquireAccess<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: Notifier<Value, AccessInput = RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
        RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone, 
        AccessResult: AccessorResult<'a, Value>
{
    type Output = Result<AccessResult, Notifyee::Error>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.waiter.lock().is_ready_to_retry() {
            let result = self.notifyee.acquire_access(self.input.clone());
            match result {
                Ok(result) => {
                    return Poll::Ready(Ok(result))
                },
                Err(error) => {
                    if !self.filter.retry(&error) {
                        return Poll::Ready(Err(error))
                    }
                }
            }
        }

        self.waiter.lock().set_waiting_to_retry(cx.waker().clone());

        Poll::Pending
    }
}

impl<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Drop for FutureAcquireAccess<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: Notifier<Value, AccessInput = RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
{
    fn drop(&mut self) {
        self.notifyee.unregister_waiter(&self.input, &self.waiter);
    }
}
