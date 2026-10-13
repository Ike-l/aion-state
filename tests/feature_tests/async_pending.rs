use std::{future::{poll_fn, Future}, sync::Arc, task::Poll};

use aion_state::prelude::{
    AccessorResult, AsyncFutureAcquireAccess, AsyncNotifier, Notifier, PermissiveFilter,
    RegistryOwnedAcquireAccess, Waiter,
};
#[cfg(feature = "releaser")]
use aion_state::prelude::{
    AsyncFutureAcquireReleasedAccess, AsyncNotifiedReleaser, AsyncReleaser, Release,
    Releaser, ReleasingResult, RegistryReleasingRelease,
};
use crate::default::prelude::{Access, ReserverId, Password, Resource, ResourceId};

type Input = RegistryOwnedAcquireAccess<ReserverId, Password, ResourceId, Access, Password>;
type Value = crate::default::storages::registry_storage::ResourceWrapper<'static, Resource>;

struct Output;

impl AccessorResult<Value> for Output {
    fn to_shared(_: Value) -> Self { Self }
    fn to_owned(_: Value) -> Self { Self }
    fn new_unique(_: Value) -> Self { Self }
}

struct PendingAsyncSource {
    waiter: Arc<parking_lot::Mutex<Waiter>>,
}

impl PendingAsyncSource {
    fn new() -> Self {
        Self { waiter: Arc::new(parking_lot::Mutex::new(Waiter::default())) }
    }

    fn allow_retry(&self) {
        let mut waiter = self.waiter.lock();
        waiter.set_ready_to_retry();
        waiter.wake();
    }
}

impl<'a> Notifier<'a, Value> for PendingAsyncSource {
    type AccessInput = Input;
    type Error = ();

    fn register_waiter(&self, _: Input) -> Arc<parking_lot::Mutex<Waiter>> {
        self.waiter.clone()
    }

    fn unregister_waiter(&self, _: &Input, _: &Arc<parking_lot::Mutex<Waiter>>) {}

    fn acquire_access<AR: AccessorResult<Value>>(&'a self, _: Input) -> Result<AR, ()> {
        Err(())
    }
}

impl<'a> AsyncNotifier<'a, Value> for PendingAsyncSource {
    fn async_acquire_access<AR: AccessorResult<Value>>(
        &'a self,
        _: Input,
    ) -> impl Future<Output = Result<AR, ()>> + 'a {
        poll_fn(|_| Poll::Pending)
    }
}

#[cfg(feature = "releaser")]
impl Release for PendingAsyncSource {
    type ReleaseInput = RegistryReleasingRelease<ResourceId, Access>;

    fn release_access(&self, _: &Self::ReleaseInput) {}
}

#[cfg(feature = "releaser")]
impl<'a> Releaser<'a, Value> for PendingAsyncSource {
    type Error = ();
    type AccessInput = Input;

    fn acquire_released_access<AR: AccessorResult<Value>>(
        self: &'a Arc<Self>,
        _: Input,
    ) -> Result<ReleasingResult<'a, Value, AR, Self>, ()> {
        Err(())
    }
}

#[cfg(feature = "releaser")]
impl<'a> AsyncReleaser<'a, Value> for PendingAsyncSource {
    fn async_acquire_released_access<AR: AccessorResult<Value>>(
        self: &'a Arc<Self>,
        _: Input,
    ) -> impl Future<Output = Result<ReleasingResult<'a, Value, AR, Self>, ()>> + 'a {
        poll_fn(|_| Poll::Pending)
    }
}

#[cfg(feature = "releaser")]
impl<'a> AsyncNotifiedReleaser<'a, Value, Input, ()> for PendingAsyncSource {}

fn input() -> Input {
    RegistryOwnedAcquireAccess {
        user_details: None,
        resource_id: ResourceId::new_label("pending-inner-future"),
        access: Access::Shared(1),
        password: None,
    }
}

#[test]
fn notifier_future_remains_pending_when_its_inner_acquire_is_pending() {
    let notifier = PendingAsyncSource::new();
    let input = input();
    let mut future = std::pin::pin!(AsyncFutureAcquireAccess::<
        Value, _, PermissiveFilter<()>, ReserverId, Password, ResourceId, Access, Password, Output,
    >::new(&notifier, input, PermissiveFilter::default()));
    let (waker, wake_count) = super::counting_waker();
    let mut context = std::task::Context::from_waker(&waker);

    assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
    assert_eq!(0, wake_count.count());
    notifier.allow_retry();
    assert_eq!(1, wake_count.count());
    assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
    assert_eq!(1, wake_count.count());
}

#[cfg(feature = "releaser")]
#[test]
fn notified_releaser_future_remains_pending_when_its_inner_acquire_is_pending() {
    let notifier = Arc::new(PendingAsyncSource::new());
    let input = input();
    let mut future = std::pin::pin!(AsyncFutureAcquireReleasedAccess::<
        Value, (), _, PermissiveFilter<()>, ReserverId, Password, ResourceId, Access, Password, Output,
    >::new(&notifier, input, PermissiveFilter::default()));
    let (waker, wake_count) = super::counting_waker();
    let mut context = std::task::Context::from_waker(&waker);

    assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
    assert_eq!(0, wake_count.count());
    notifier.allow_retry();
    assert_eq!(1, wake_count.count());
    assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
    assert_eq!(1, wake_count.count());
}
