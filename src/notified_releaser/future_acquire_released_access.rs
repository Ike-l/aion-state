use std::{marker::PhantomData, pin::Pin, task::{Context, Poll}};

use crate::prelude::{AccessFilter, AccessorResult, NotifiedReleaser, RegistryOwnedAcquireAccess, ReleasingResult, Waiter, sync::{Arc, Mutex}};

pub struct FutureAcquireReleasedAccess<'a,
    Value,
    Error,
    Notifyee: NotifiedReleaser<Value, RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>, Error>,
    Filter: AccessFilter<Error = Error>,
    Id, IdPassword, ResourceId, Access, Password,
    AccessResult
> {
    notifyee: &'a Arc<Notifyee>,
    input: RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
    filter: Filter,
    waiter: Arc<Mutex<Waiter>>,
    _r: PhantomData<AccessResult>,
    _v: PhantomData<Value>
}

impl<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> FutureAcquireReleasedAccess<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: NotifiedReleaser<Value, RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>, Error>,
        Filter: AccessFilter<Error = Error>,
        RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone,
{
    pub fn new(
        notifyee: &'a Arc<Notifyee>,
        input: RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
        filter: Filter,
    ) -> Self {
        let waiter = notifyee.register_waiter(input.clone());
        Self { notifyee, input, filter, waiter, _r: Default::default(), _v: Default::default() }
    }
}

impl<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Future for FutureAcquireReleasedAccess<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: NotifiedReleaser<Value, RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>, Error>,
        Filter: AccessFilter<Error = Error>,
        RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone, 
        AccessResult: AccessorResult<'a, Value>
{
    type Output = Result<ReleasingResult<Value, AccessResult, Notifyee>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.waiter.lock().is_ready_to_retry() {
            let result = self.notifyee.acquire_released_access(self.input.clone());
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

impl<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Drop for FutureAcquireReleasedAccess<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: NotifiedReleaser<Value, RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>, Error>,
        Filter: AccessFilter<Error = Error>,
{
    fn drop(&mut self) {
        self.notifyee.unregister_waiter(&self.input, &self.waiter);
    }
}
