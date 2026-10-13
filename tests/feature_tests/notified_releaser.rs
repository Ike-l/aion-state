use std::{future::Future, sync::Arc, task::{Context, Poll}};

use aion_state::prelude::{
    AccessFilter, RegistryOwnedAcquireAccess, RegistryReplacement,
    Releaser, SynchronisedRegistryAcquireAccessError,
};
use aion_state::notified_releaser::future_acquire_released_access::FutureAcquireReleasedAccess;

use crate::{TestRegistry, create_registry, default::prelude::*};
use super::counting_waker;

struct RetryAcquire;
impl AccessFilter for RetryAcquire {
    type Error = SynchronisedRegistryAcquireAccessError;
    fn retry(&self, _error: &Self::Error) -> bool { true }
}

struct StopOnError;
impl AccessFilter for StopOnError {
    type Error = SynchronisedRegistryAcquireAccessError;
    fn retry(&self, _error: &Self::Error) -> bool { false }
}

#[test]
fn notified_releaser_future_waits_for_and_releases_conflicting_access() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let registry = Arc::new(create_registry(None));
    let resource_id = ResourceId::new_label("notified-releaser");
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
    let mut future = std::pin::pin!(FutureAcquireReleasedAccess::<
        ResourceWrapper<'_, Resource>,
        SynchronisedRegistryAcquireAccessError,
        TestRegistry,
        RetryAcquire,
        ReserverId,
        Password,
        ResourceId,
        Access,
        Password,
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
    assert!(wake_count.count() > 0, "dropping the guard should wake the registered waiter");
    let unique = match future.as_mut().poll(&mut context) {
        Poll::Ready(Ok(unique)) => unique,
        Poll::Ready(Err(error)) => panic!("notified releaser failed after wake: {error:?}"),
        Poll::Pending => panic!("releaser drop did not wake future"),
    };
    drop(unique);
    drop(future);
    assert_eq!(Some(Access::Shared(0)), registry.get_access(&aion_state::prelude::ReceptionGetAccess { access_id: &resource_id }));
}

#[test]
fn notified_releaser_returns_authentication_error_when_filter_stops() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let registry = Arc::new(create_registry(None));
    let resource_id = ResourceId::new_label("notified-releaser-error");
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

    let mut future = std::pin::pin!(FutureAcquireReleasedAccess::<
        ResourceWrapper<'_, Resource>,
        SynchronisedRegistryAcquireAccessError,
        TestRegistry,
        StopOnError,
        ReserverId,
        Password,
        ResourceId,
        Access,
        Password,
        AccessResult<ResourceWrapper<'_, Resource>>,
    >::new(
        &registry,
        RegistryOwnedAcquireAccess {
            user_details: Some((ReserverId::new("unregistered"), Password::new(41))),
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
    let mut retry_future = std::pin::pin!(FutureAcquireReleasedAccess::<
        ResourceWrapper<'_, Resource>,
        SynchronisedRegistryAcquireAccessError,
        TestRegistry,
        RetryAcquire,
        ReserverId,
        Password,
        ResourceId,
        Access,
        Password,
        AccessResult<ResourceWrapper<'_, Resource>>,
    >::new(
        &registry,
        RegistryOwnedAcquireAccess {
            user_details: Some((ReserverId::new("retry-unregistered"), Password::new(43))),
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
