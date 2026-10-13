use std::{future::Future, task::{Context, Poll}};

use aion_state::prelude::{
    AccessFilter, AsyncFutureAcquireAccess, RegistryAcquireAccess, RegistryOwnedAcquireAccess,
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

struct StopOnError;

impl AccessFilter for StopOnError {
    type Error = SynchronisedRegistryAcquireAccessError;

    fn retry(&self, _error: &Self::Error) -> bool {
        false
    }
}

#[test]
fn async_notifier_retries_acquisition_after_release() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("feature-async-notifier");
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

    let mut future = std::pin::pin!(AsyncFutureAcquireAccess::<
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
    assert!(wake_count.count() > 0, "releasing the access should wake the async waiter");

    let unique = match future.as_mut().poll(&mut context) {
        Poll::Ready(Ok(access)) => access,
        Poll::Ready(Err(error)) => panic!("async notifier retry failed: {error:?}"),
        Poll::Pending => panic!("async notifier did not wake after release"),
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
fn async_notifier_handles_cancelled_and_multiple_waiters() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    type ResultValue<'a> = AccessResult<ResourceWrapper<'a, Resource>>;
    type WaitFuture<'a, Filter> = AsyncFutureAcquireAccess<
        'a, ResourceWrapper<'a, Resource>, crate::TestRegistry, Filter,
        ReserverId, Password, ResourceId, Access, Password, ResultValue<'a>,
    >;

    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("async-notifier-multiple");
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    }).ok());
    let unique = registry.acquire_access::<ResultValue<'_>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Unique,
        password: None,
    }).unwrap();
    drop(unique);

    let input = || RegistryOwnedAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    };
    let mut cancelled = Box::pin(WaitFuture::new(&registry, input(), RetryAcquire));
    let mut first = Box::pin(WaitFuture::new(&registry, input(), RetryAcquire));
    let mut second = Box::pin(WaitFuture::new(&registry, input(), RetryAcquire));
    let (cancelled_waker, cancelled_wakes) = counting_waker();
    let (first_waker, first_wakes) = counting_waker();
    let (second_waker, second_wakes) = counting_waker();
    let mut cancelled_context = Context::from_waker(&cancelled_waker);
    let mut first_context = Context::from_waker(&first_waker);
    let mut second_context = Context::from_waker(&second_waker);
    assert!(matches!(cancelled.as_mut().poll(&mut cancelled_context), Poll::Pending));
    assert!(matches!(first.as_mut().poll(&mut first_context), Poll::Pending));
    assert!(matches!(second.as_mut().poll(&mut second_context), Poll::Pending));
    drop(cancelled);

    assert!(matches!(unsafe {
        registry.release_access(&RegistryReleaseAccess { resource_id: &resource_id, access: &Access::Unique })
    }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
    assert_eq!(0, cancelled_wakes.count(), "cancelled async waiter must not be woken");
    assert!(first_wakes.count() >= 1, "first async waiter should be woken");
    assert!(second_wakes.count() >= 1, "second async waiter should be woken");

    let first_access = match first.as_mut().poll(&mut first_context) {
        Poll::Ready(Ok(access)) => access,
        Poll::Ready(Err(error)) => panic!("first async waiter failed: {error:?}"),
        Poll::Pending => panic!("first async waiter was not notified"),
    };
    let second_access = match second.as_mut().poll(&mut second_context) {
        Poll::Ready(Ok(access)) => access,
        Poll::Ready(Err(error)) => panic!("second async waiter failed: {error:?}"),
        Poll::Pending => panic!("second async waiter was not notified"),
    };
    drop(first_access);
    drop(second_access);
    drop(first);
    drop(second);
    for _ in 0..2 {
        assert!(matches!(unsafe {
            registry.release_access(&RegistryReleaseAccess { resource_id: &resource_id, access: &Access::Shared(1) })
        }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
    }
}

#[test]
fn async_notifier_filter_can_stop_after_authentication_failure() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("async-notifier-filter-error");
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    }).ok());
    let shared = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }).unwrap();
    drop(shared);

    let invalid_id = ReserverId::new("async-unregistered".to_string());
    let invalid_password = Password::new(3);
    let mut future = std::pin::pin!(AsyncFutureAcquireAccess::<
        ResourceWrapper<'_, Resource>, _, StopOnError, _, _, ResourceId, Access, _,
        AccessResult<ResourceWrapper<'_, Resource>>,
    >::new(
        &registry,
        RegistryOwnedAcquireAccess {
            user_details: Some((invalid_id, invalid_password)),
            resource_id: resource_id.clone(),
            access: Access::Shared(1),
            password: None,
        },
        StopOnError,
    ));
    let (waker, wake_count) = counting_waker();
    let mut context = Context::from_waker(&waker);
    assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
    assert!(matches!(unsafe {
        registry.release_access(&RegistryReleaseAccess { resource_id: &resource_id, access: &Access::Shared(1) })
    }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
    assert!(matches!(future.as_mut().poll(&mut context), Poll::Ready(Err(SynchronisedRegistryAcquireAccessError::VerificationFailure))));

    let shared = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }).unwrap();
    drop(shared);
    let mut retry_future = std::pin::pin!(AsyncFutureAcquireAccess::<
        ResourceWrapper<'_, Resource>, _, PermissiveFilter<SynchronisedRegistryAcquireAccessError>,
        _, _, ResourceId, Access, _, AccessResult<ResourceWrapper<'_, Resource>>,
    >::new(
        &registry,
        RegistryOwnedAcquireAccess {
            user_details: Some((ReserverId::new("async-retry-unregistered"), Password::new(11))),
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
