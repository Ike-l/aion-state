use std::{marker::PhantomData, pin::Pin, sync::atomic::{AtomicBool, Ordering}, task::Poll};

use crate::prelude::{AccessFilter, AccessorResult, AsyncNotifier, RegistryOwnedAcquireAccess, Waiter, sync::{Arc, Mutex}};

pub struct AsyncFutureAcquireAccess<'a,
    Value,
    Notifyee: AsyncNotifier<Value, AccessInput = RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>, 
    Filter: AccessFilter<Error = Notifyee::Error>,
    Id, IdPassword, ResourceId, Access, Password,
    AccessResult
> {
    notifyee: &'a Notifyee,
    input: RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
    filter: Filter,
    waiter: Arc<Mutex<Waiter>>,
    finished: AtomicBool,
    acquire_future: Option<Pin<Box<dyn Future<Output = Result<AccessResult, Notifyee::Error>> + 'a>>>,
    _r: PhantomData<AccessResult>,
    _v: PhantomData<Value>
}

impl<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> AsyncFutureAcquireAccess<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: AsyncNotifier<Value, AccessInput = RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
        RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone
{
    pub fn new(
        notifyee: &'a Notifyee,
        input: RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
        filter: Filter,
    ) -> Self {
        let waiter = notifyee.register_waiter(input.clone());
        Self { 
            notifyee, 
            input, 
            filter, 
            waiter, 
            finished: AtomicBool::new(false), 
            acquire_future: None,
            _r: Default::default(), 
            _v: Default::default() 
        }
    }
}

impl<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Unpin for AsyncFutureAcquireAccess<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: AsyncNotifier<Value, AccessInput = RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
        RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone, 
        AccessResult: AccessorResult<'a, Value>
{}

impl<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Future for AsyncFutureAcquireAccess<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: AsyncNotifier<Value, AccessInput = RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
        RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone, 
        AccessResult: AccessorResult<'a, Value>
{
    type Output = Result<AccessResult, Notifyee::Error>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.finished.load(Ordering::Acquire) {
            panic!("I hope i never have to deal with this");
        }

        // can drop guard because it doesnt actually matter if the waiter changes after this
        let should_retry = {
            let mut waiter = self.waiter.lock();
            waiter.set_waker(cx.waker().clone());
            waiter.is_ready_to_retry()
        };

        if self.acquire_future.is_none() && should_retry {
            let future = self.notifyee.async_acquire_access(self.input.clone());
            self.acquire_future = Some(Box::pin(future));
        }

        if let Some(future) = &mut self.acquire_future {
            match future.as_mut().poll(cx) {
                Poll::Ready(Ok(result)) => {
                    self.finished.store(true, Ordering::Release);
                    return Poll::Ready(Ok(result));
                }

                Poll::Ready(Err(error)) => {
                    self.acquire_future = None;

                    let mut waiter = self.waiter.lock();

                    if self.filter.retry(&error) {
                        waiter.set_waiting_to_retry();
                    } else {
                        return Poll::Ready(Err(error));
                    }
                }

                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }

        Poll::Pending
    }
}

impl<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Drop for AsyncFutureAcquireAccess<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: AsyncNotifier<Value, AccessInput = RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
{
    fn drop(&mut self) {
        self.notifyee.unregister_waiter(&self.input, &self.waiter);
    }
}
