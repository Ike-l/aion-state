use std::{future::Future, task::{Context, Poll}};

use aion_state::prelude::{
    AccessFilter, FutureAcquireAccess, NotifyQueue, RegistryAcquireAccess, RegistryOwnedAcquireAccess,
    PermissiveFilter, RegistryReleaseAccess, RegistryReplacement, SynchronisedRegistryAcquireAccessError,
};

use crate::{create_registry, default::prelude::*};
use super::counting_waker;

struct RetryAcquire;

impl AccessFilter for RetryAcquire {
    type Error = SynchronisedRegistryAcquireAccessError;

    fn retry(&self, _error: &Self::Error) -> bool {
        true
    }
}

struct StopOnFirstError;

impl AccessFilter for StopOnFirstError {
    type Error = SynchronisedRegistryAcquireAccessError;

    fn retry(&self, _error: &Self::Error) -> bool {
        false
    }
}

#[test]
fn notify_queue_unregister_preserves_unmatched_waiters() {
    let id = ResourceId::new_label("notify-queue-unregister");
    let missing_id = ResourceId::new_label("notify-queue-missing");
    let other_id = ResourceId::new_label("notify-queue-other");
    let mut queue = NotifyQueue::default();
    let waiter = queue.register(id.clone());
    let unmatched = queue.register(other_id);
    let (waker, wake_count) = counting_waker();
    waiter.lock().set_waiting_to_retry(waker);

    queue.unregister(&missing_id, &unmatched);
    queue.unregister(&id, &unmatched);
    queue.wake(&id);
    assert_eq!(1, wake_count.count());

    queue.unregister(&id, &waiter);
    queue.wake(&id);
    assert_eq!(1, wake_count.count());
}

#[test]
fn notifier_filter_can_stop_after_authentication_failure() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("notifier-filter-error");
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    }).ok());
    let held = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }).unwrap();
    drop(held);

    let invalid_id = ReserverId::new("unregistered".to_string());
    let invalid_password = Password::new(9);
    let mut future = std::pin::pin!(FutureAcquireAccess::<
        ResourceWrapper<'_, Resource>,
        _,
        StopOnFirstError,
        _,
        _,
        ResourceId,
        Access,
        _,
        AccessResult<ResourceWrapper<'_, Resource>>,
    >::new(
        &registry,
        RegistryOwnedAcquireAccess {
            user_details: Some((invalid_id, invalid_password)),
            resource_id: resource_id.clone(),
            access: Access::Shared(1),
            password: None,
        },
        StopOnFirstError,
    ));
    let (waker, wake_count) = counting_waker();
    let mut context = Context::from_waker(&waker);
    assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
    assert!(matches!(unsafe {
        registry.release_access(&RegistryReleaseAccess {
            resource_id: &resource_id,
            access: &Access::Shared(1),
        })
    }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
    assert!(matches!(future.as_mut().poll(&mut context), Poll::Ready(Err(SynchronisedRegistryAcquireAccessError::VerificationFailure))));

    let shared = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }).unwrap();
    drop(shared);
    let mut retry_future = std::pin::pin!(FutureAcquireAccess::<
        ResourceWrapper<'_, Resource>, _, PermissiveFilter<SynchronisedRegistryAcquireAccessError>,
        _, _, ResourceId, Access, _, AccessResult<ResourceWrapper<'_, Resource>>,
    >::new(
        &registry,
        RegistryOwnedAcquireAccess {
            user_details: Some((ReserverId::new("retry-unregistered"), Password::new(10))),
            resource_id: resource_id.clone(),
            access: Access::Shared(1),
            password: None,
        },
        PermissiveFilter::default(),
    ));
    assert!(matches!(retry_future.as_mut().poll(&mut context), Poll::Pending));
    assert!(matches!(unsafe {
        registry.release_access(&RegistryReleaseAccess { resource_id: &resource_id, access: &Access::Shared(1) })
    }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
    assert!(wake_count.count() > 0);
    assert!(matches!(retry_future.as_mut().poll(&mut context), Poll::Pending));
}

#[test]
fn notifier_future_retries_after_the_current_access_is_released() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("feature-notifier-retry");
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    }).ok());

    let shared_access = Access::Shared(1);
    let shared = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: shared_access.clone(),
        password: None,
    }).unwrap();
    drop(shared);

    let mut future = std::pin::pin!(FutureAcquireAccess::<
        ResourceWrapper<'_, Resource>,
        _,
        PermissiveFilter<SynchronisedRegistryAcquireAccessError>,
        _,
        _,
        ResourceId,
        Access,
        _,
        AccessResult<ResourceWrapper<'_, Resource>>,
    >::new(
        &registry,
        RegistryOwnedAcquireAccess {
            user_details: None,
            resource_id: resource_id.clone(),
            access: Access::Unique,
            password: None,
        },
        PermissiveFilter::default(),
    ));
    let (waker, wake_count) = counting_waker();
    let mut context = Context::from_waker(&waker);
    assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));

    assert!(matches!(unsafe {
        registry.release_access(&RegistryReleaseAccess {
            resource_id: &resource_id,
            access: &shared_access,
        })
    }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
    assert!(wake_count.count() > 0, "releasing the access should wake the registered waiter");

    let unique = match future.as_mut().poll(&mut context) {
        Poll::Ready(Ok(access)) => access,
        Poll::Ready(Err(error)) => panic!("notifier retry failed: {error:?}"),
        Poll::Pending => panic!("notifier did not wake the pending future"),
    };
    drop(unique);
    assert!(matches!(unsafe {
        registry.release_access(&RegistryReleaseAccess {
            resource_id: &resource_id,
            access: &Access::Unique,
        })
    }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
}

#[test]
fn notifier_supports_cancellation_and_multiple_waiters_with_different_filters() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    type ResultValue<'a> = AccessResult<ResourceWrapper<'a, Resource>>;
    type RetryFuture<'a> = FutureAcquireAccess<
        'a, ResourceWrapper<'a, Resource>, crate::TestRegistry, RetryAcquire,
        ReserverId, Password, ResourceId, Access, Password, ResultValue<'a>,
    >;
    type StopFuture<'a> = FutureAcquireAccess<
        'a, ResourceWrapper<'a, Resource>, crate::TestRegistry, StopOnFirstError,
        ReserverId, Password, ResourceId, Access, Password, ResultValue<'a>,
    >;

    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("notifier-multiple-waiters");
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    }).ok());
    let shared = registry.acquire_access::<ResultValue<'_>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }).unwrap();
    drop(shared);

    let make_input = || RegistryOwnedAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Unique,
        password: None,
    };
    let mut cancelled = Box::pin(RetryFuture::new(&registry, make_input(), RetryAcquire));
    let mut retrying = Box::pin(RetryFuture::new(&registry, make_input(), RetryAcquire));
    let mut stopping = Box::pin(StopFuture::new(&registry, make_input(), StopOnFirstError));
    let (cancelled_waker, cancelled_wakes) = counting_waker();
    let (retrying_waker, retrying_wakes) = counting_waker();
    let (stopping_waker, stopping_wakes) = counting_waker();
    let mut cancelled_context = Context::from_waker(&cancelled_waker);
    let mut retrying_context = Context::from_waker(&retrying_waker);
    let mut stopping_context = Context::from_waker(&stopping_waker);
    assert!(matches!(cancelled.as_mut().poll(&mut cancelled_context), Poll::Pending));
    assert!(matches!(retrying.as_mut().poll(&mut retrying_context), Poll::Pending));
    assert!(matches!(stopping.as_mut().poll(&mut stopping_context), Poll::Pending));
    drop(cancelled);

    assert!(matches!(unsafe {
        registry.release_access(&RegistryReleaseAccess {
            resource_id: &resource_id,
            access: &Access::Shared(1),
        })
    }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
    assert_eq!(0, cancelled_wakes.count(), "cancelled waiter must not be woken");
    assert!(retrying_wakes.count() >= 1, "retrying waiter should be woken");
    assert!(stopping_wakes.count() >= 1, "stopping waiter should be woken");

    let unique = match retrying.as_mut().poll(&mut retrying_context) {
        Poll::Ready(Ok(unique)) => unique,
        Poll::Ready(Err(error)) => panic!("first notified waiter failed: {error:?}"),
        Poll::Pending => panic!("first waiter was not notified"),
    };
    assert!(matches!(stopping.as_mut().poll(&mut stopping_context), Poll::Ready(Err(_))));
    drop(unique);
    drop(retrying);
    drop(stopping);
    assert!(matches!(unsafe {
        registry.release_access(&RegistryReleaseAccess {
            resource_id: &resource_id,
            access: &Access::Unique,
        })
    }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
}
