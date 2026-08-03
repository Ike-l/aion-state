use std::{marker::PhantomData, sync::atomic::{AtomicBool, Ordering}, task::Poll};

use crate::prelude::{AccessFilter, AccessorResult, Notifier, RegistryNotifiedAcquireAccess, Waiter, sync::{Arc, Mutex}};

pub struct FutureAcquireAccess<'a,
    Value,
    Notifyee: Notifier<Value, AccessInput = RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>, 
    Filter: AccessFilter<Error = Notifyee::Error>,
    Id, IdPassword, ResourceId, Access, Password,
    AccessResult
> {
    notifyee: &'a Notifyee,
    input: RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
    filter: Filter,
    waiter: Arc<Mutex<Waiter>>,
    finished: AtomicBool,
    _r: PhantomData<AccessResult>,
    _v: PhantomData<Value>
}

impl<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> FutureAcquireAccess<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: Notifier<Value, AccessInput = RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
        RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone
{
    pub fn new(
        notifyee: &'a Notifyee,
        input: RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>,
        filter: Filter,
    ) -> Self {
        let waiter = notifyee.register_waiter(input.clone());
        Self { notifyee, input, filter, waiter, finished: AtomicBool::new(false), _r: Default::default(), _v: Default::default() }
    }
}

impl<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Future for FutureAcquireAccess<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: Notifier<Value, AccessInput = RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
        RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>: Clone, 
        AccessResult: AccessorResult<'a, Value>
{
    type Output = Result<AccessResult, Notifyee::Error>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.finished.load(Ordering::Acquire) {
            panic!("I hope i never have to deal with this");
        }

        // can drop guard because it doesnt actually matter if the waiter changes after this
        let should_retry = {
            let mut waiter = self.waiter.lock();
            waiter.set_waker(cx.waker().clone());
            waiter.is_ready_to_retry()
        };

        if should_retry {
            let result = self.notifyee.acquire_access(self.input.clone());
            match result {
                Ok(result) => {
                    self.finished.store(true, Ordering::Release);
                    return Poll::Ready(Ok(result))
                },
                Err(error) => {
                    if self.filter.retry(&error) {
                        self.waiter.lock().set_waiting_to_retry();
                    } else {
                        return Poll::Ready(Err(error))
                    }
                }
            }
        }

        Poll::Pending
    }
}

impl<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> Drop for FutureAcquireAccess<'a, Value, Notifyee, Filter, Id, IdPassword, ResourceId, Access, Password, AccessResult> 
    where 
        Notifyee: Notifier<Value, AccessInput = RegistryNotifiedAcquireAccess<Id, IdPassword, ResourceId, Access, Password>>,
        Filter: AccessFilter<Error = Notifyee::Error>,
{
    fn drop(&mut self) {
        self.notifyee.unregister_waiter(&self.input, &self.waiter);
    }
}
