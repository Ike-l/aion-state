use std::{marker::PhantomData, pin::Pin, task::Poll};

use crate::prelude::{AccessFilter, AccessorResult, AsyncNotifiedReleaser, RegistryOwnedAcquireAccess, ReleasingResult, Waiter, sync::{Arc, Mutex}};

pub struct AsyncFutureAcquireReleasedAccess<'a,
    Value,
    Error,
    Notifyee: AsyncNotifiedReleaser<Value, RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>, Error>, 
    Filter: AccessFilter<Error = Error>,
    Id, IdPassword, ResourceId, Access, Password,
    AccessResult
> {
    notifyee: &'a Arc<Notifyee>,
    input: RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
    filter: Filter,
    waiter: Arc<Mutex<Waiter>>,
    acquire_future: Option<Pin<Box<dyn Future<Output = Result<ReleasingResult<Value, AccessResult, Notifyee>, Error>> + 'a>>>,
    _r: PhantomData<AccessResult>,
    _v: PhantomData<Value>
}

impl<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> AsyncFutureAcquireReleasedAccess<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: AsyncNotifiedReleaser<Value, RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>, Error>,
        Filter: AccessFilter<Error = Error>,
        RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone
{
    pub fn new(
        notifyee: &'a Arc<Notifyee>,
        input: RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
        filter: Filter,
    ) -> Self {
        let waiter = notifyee.register_waiter(input.clone());
        Self { 
            notifyee, 
            input, 
            filter, 
            waiter, 
            acquire_future: None,
            _r: Default::default(), 
            _v: Default::default() 
        }
    }
}

impl<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Unpin for AsyncFutureAcquireReleasedAccess<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: AsyncNotifiedReleaser<Value, RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>, Error>,
        Filter: AccessFilter<Error = Error>,
        RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone, 
        AccessResult: AccessorResult<'a, Value>
{}

impl<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Future for AsyncFutureAcquireReleasedAccess<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: AsyncNotifiedReleaser<Value, RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>, Error>,
        Filter: AccessFilter<Error = Error>,
        RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone, 
        AccessResult: AccessorResult<'a, Value>
{
    type Output = Result<ReleasingResult<Value, AccessResult, Notifyee>, Error>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.acquire_future.is_none() && self.waiter.lock().is_ready_to_retry() {
            let future = self.notifyee.async_acquire_released_access(self.input.clone());
            self.acquire_future = Some(Box::pin(future));
        }

        if let Some(future) = &mut self.acquire_future {
            match future.as_mut().poll(cx) {
                Poll::Ready(Ok(result)) => {
                    return Poll::Ready(Ok(result));
                }

                Poll::Ready(Err(error)) => {
                    self.acquire_future = None;

                    if !self.filter.retry(&error) {
                        return Poll::Ready(Err(error));
                    }
                }

                Poll::Pending => ()
            }
        }

        self.waiter.lock().set_waiting_to_retry(cx.waker().clone());

        Poll::Pending
    }
}

impl<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Drop for AsyncFutureAcquireReleasedAccess<'a, Value, Error, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: AsyncNotifiedReleaser<Value, RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>, Error>,
        Filter: AccessFilter<Error = Error>,
{
    fn drop(&mut self) {
        self.notifyee.unregister_waiter(&self.input, &self.waiter);
    }
}
