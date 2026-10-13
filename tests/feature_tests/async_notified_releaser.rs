use std::{future::Future, sync::Arc, task::{Context, Poll}};

use aion_state::prelude::{
    AccessFilter, AsyncFutureAcquireReleasedAccess, RegistryOwnedAcquireAccess,
    RegistryReplacement, Releaser, SynchronisedRegistryAcquireAccessError,
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
fn async_notified_releaser_resumes_after_guard_drop() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let registry = Arc::new(create_registry(None));
    let resource_id = ResourceId::new_label("feature-async-notified-release");
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    }).ok());

    let shared = registry.acquire_released_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryOwnedAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }).unwrap();

    let mut future = std::pin::pin!(AsyncFutureAcquireReleasedAccess::<
        ResourceWrapper<'_, Resource>,
        SynchronisedRegistryAcquireAccessError,
        _,
        RetryAcquire,
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
        RetryAcquire,
    ));
    let (waker, wake_count) = counting_waker();
    let mut context = Context::from_waker(&waker);
    assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));

    drop(shared);
    assert!(wake_count.count() > 0, "dropping the guard should wake the async waiter");
    let unique = match future.as_mut().poll(&mut context) {
        Poll::Ready(Ok(access)) => access,
        Poll::Ready(Err(error)) => panic!("async released acquisition failed: {error:?}"),
        Poll::Pending => panic!("dropping the guard did not wake the async waiter"),
    };
    drop(unique);
    assert_eq!(Some(Access::Shared(0)), registry.get_access(&aion_state::prelude::ReceptionGetAccess { access_id: &resource_id }));
}

#[test]
fn async_notified_releaser_returns_authentication_error_when_filter_stops() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let registry = Arc::new(create_registry(None));
    let resource_id = ResourceId::new_label("async-notified-releaser-error");
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("value".to_string())),
        password: None,
    }).ok());
    let shared = registry.acquire_released_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryOwnedAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }).unwrap();

    let mut future = std::pin::pin!(AsyncFutureAcquireReleasedAccess::<
        ResourceWrapper<'_, Resource>,
        SynchronisedRegistryAcquireAccessError,
        _,
        StopOnError,
        _,
        _,
        ResourceId,
        Access,
        _,
        AccessResult<ResourceWrapper<'_, Resource>>,
    >::new(
        &registry,
        RegistryOwnedAcquireAccess {
            user_details: Some((ReserverId::new("async-unregistered"), Password::new(42))),
            resource_id: resource_id.clone(),
            access: Access::Shared(1),
            password: None,
        },
        StopOnError,
    ));
    let (waker, wake_count) = counting_waker();
    let mut context = Context::from_waker(&waker);
    assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));

    drop(shared);
    assert!(wake_count.count() > 0);
    assert!(matches!(future.as_mut().poll(&mut context), Poll::Ready(Err(SynchronisedRegistryAcquireAccessError::VerificationFailure))));
    drop(future);

    let shared = registry.acquire_released_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryOwnedAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }).unwrap();
    let mut retry_future = std::pin::pin!(AsyncFutureAcquireReleasedAccess::<
        ResourceWrapper<'_, Resource>,
        SynchronisedRegistryAcquireAccessError,
        _,
        RetryAcquire,
        _,
        _,
        ResourceId,
        Access,
        _,
        AccessResult<ResourceWrapper<'_, Resource>>,
    >::new(
        &registry,
        RegistryOwnedAcquireAccess {
            user_details: Some((ReserverId::new("async-retry-unregistered"), Password::new(44))),
            resource_id: resource_id.clone(),
            access: Access::Shared(1),
            password: None,
        },
        RetryAcquire,
    ));
    assert!(matches!(retry_future.as_mut().poll(&mut context), Poll::Pending));
    let wake_count_before_release = wake_count.count();
    drop(shared);
    assert!(wake_count.count() > wake_count_before_release);
    assert!(matches!(retry_future.as_mut().poll(&mut context), Poll::Pending));
}
