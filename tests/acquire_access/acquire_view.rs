use aion_state::prelude::{
    RegistryAcquireView, RegistryOwn, RegistryRegister, RegistryReplacement,
    SynchronisedRegistryAcquireAccessError, SynchronisedRegistryAcquireViewError,
};
#[cfg(feature = "releaser")]
use aion_state::prelude::RegistryOwnedAcquireView;
#[cfg(all(feature = "notifier", feature = "releaser"))]
use aion_state::prelude::ViewReleaser;
#[cfg(all(feature = "async", feature = "releaser"))]
use aion_state::prelude::AsyncViewReleaser;

use crate::default::prelude::*;

use crate::create_registry;

fn registry_with_two_resources() -> (crate::TestRegistry, [ResourceId; 2], [Resource; 2]) {
    let registry = create_registry(None);
    let resource_ids = [ResourceId::new_label("1"), ResourceId::new_label("2")];
    let resources = [Resource::new("resource1".to_string()), Resource::new("resource2".to_string())];

    for (resource_id, resource) in resource_ids.iter().zip(&resources) {
        assert!(registry.checked_replace(RegistryReplacement {
            user_details: None,
            access: &Access::Replace,
            resource_id: resource_id.clone(),
            resource: Some(resource.clone()),
            password: None,
        }).ok());
    }

    (registry, resource_ids, resources)
}

#[test]
fn can_acquire_view_shared() {
    use crate::default::storages::registry_storage::ResourceView;

    let (registry, resource_ids, resources) = registry_with_two_resources();
    let [resource_id1, resource_id2] = resource_ids;
    let [resource1, resource2] = resources;

    let result = registry.acquire_view::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: None,
        resource_access: vec![
            (resource_id1.clone(), None),
            (resource_id2.clone(), None),
        ],
        access: Access::Shared(1),
    }).unwrap();

    assert!(match result {
        AccessResult::Shared(ResourceView::Shared(view)) => {
            view.get(&resource_id1).map(|resource| **resource == resource1) == Some(true) &&
            view.get(&resource_id2).map(|resource| **resource == resource2) == Some(true)
        }
        _ => false
    });
}

#[test]
fn can_acquire_view_unique() {
    use crate::default::storages::registry_storage::ResourceView;

    let (registry, resource_ids, resources) = registry_with_two_resources();
    let [resource_id1, resource_id2] = resource_ids;
    let [resource1, resource2] = resources;

    let result = registry.acquire_view::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: None,
        resource_access: vec![
            (resource_id1.clone(), None),
            (resource_id2.clone(), None),
        ],
        access: Access::Unique,
    }).unwrap();

    assert!(match result {
        AccessResult::Unique(ResourceView::Unique(view)) => {
            view.get(&resource_id1).map(|resource| **resource == resource1) == Some(true) &&
            view.get(&resource_id2).map(|resource| **resource == resource2) == Some(true)
        }
        _ => false
    });
}

#[test]
fn cannot_acquire_view_using_replace_access() {
    use crate::default::storages::registry_storage::ResourceView;

    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("replace-view-access");
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("value".to_string())),
        password: None,
    }).ok());

    let result = registry.acquire_view::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: None,
        resource_access: vec![(resource_id, None)],
        access: Access::Replace,
    });
    assert!(matches!(result, Err(SynchronisedRegistryAcquireViewError::TriedAcquiring)));
}

#[test]
fn cannot_acquire_view_with_duplicates() {
    let registry = create_registry(None);

    let resource_id1 = ResourceId::new_label("1");
    let resource1 = Resource::new("resource1".to_string());

    let result = registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id1.clone(),
        resource: Some(resource1.clone()),
        password: None,
    });

    assert!(result.ok());

    let result = registry.acquire_view::<AccessResult<crate::default::storages::registry_storage::ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: None,
        resource_access: vec![
            (resource_id1.clone(), None),
            (resource_id1.clone(), None),
        ],
        access: Access::Shared(1),
    });

    assert!(result.is_err_and(|err| matches!(err, SynchronisedRegistryAcquireViewError::DuplicateIds(indices) if indices == vec![1])));
}

#[test]
fn reports_all_rejected_resources() {
    use crate::default::storages::registry_storage::ResourceView;

    let registry = create_registry(None);
    let resource_id1 = ResourceId::new_label("1");
    let resource_id2 = ResourceId::new_label("2");

    for resource_id in [&resource_id1, &resource_id2] {
        let result = registry.checked_replace(RegistryReplacement {
            user_details: None,
            access: &Access::Replace,
            resource_id: resource_id.clone(),
            resource: Some(Resource::new("resource".to_string())),
            password: None,
        });
        assert!(result.ok());
    }

    {
        let _view = registry.acquire_view::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
            user_details: None,
            resource_access: vec![(resource_id1.clone(), None), (resource_id2.clone(), None)],
            access: Access::Shared(1),
        }).unwrap();
    }

    let result = registry.acquire_view::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: None,
        resource_access: vec![(resource_id1, None), (resource_id2, None)],
        access: Access::Unique,
    });

    assert!(result.is_err_and(|err| matches!(
        err,
        SynchronisedRegistryAcquireViewError::Reception(errors)
            if errors.len() == 2
                && errors.iter().map(|(index, _)| *index).collect::<Vec<_>>() == vec![0, 1]
                && errors.iter().all(|(_, error)| matches!(error, aion_state::prelude::SynchronisedRegistryAcquireAccessError::AccessConflict))
    )));
}

#[test]
fn cannot_acquire_view_with_invalid_user_credentials() {
    use crate::default::storages::registry_storage::ResourceView;

    let registry = create_registry(None);
    let owner = ReserverId::new("view-owner");
    let owner_password = Password::new(71);
    let guest = ReserverId::new("view-guest");
    let guest_password = Password::new(72);
    assert!(registry.register(RegistryRegister { id: owner.clone(), password: owner_password.clone() }).ok());
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: guest_password }).ok());

    let resource_ids = [ResourceId::new_label("view-protected-a"), ResourceId::new_label("view-protected-b")];
    for resource_id in &resource_ids {
        assert!(registry.own(RegistryOwn {
            id: owner.clone(),
            password: &owner_password,
            resource_id: resource_id.clone(),
        }).ok());
        assert!(registry.checked_replace(RegistryReplacement {
            user_details: Some((&owner, &owner_password)),
            access: &Access::Replace,
            resource_id: resource_id.clone(),
            resource: Some(Resource::new("protected".to_string())),
            password: None,
        }).ok());
    }

    let result = registry.acquire_view::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: Some((&guest, &Password::new(0))),
        resource_access: resource_ids.iter().cloned().map(|id| (id, None)).collect(),
        access: Access::Shared(1),
    });
    assert!(result.is_err_and(|error| matches!(
        error,
        SynchronisedRegistryAcquireViewError::Reception(items)
            if items.len() == resource_ids.len()
                && items.iter().enumerate().all(|(index, (reported_index, error))|
                    index == *reported_index
                        && matches!(error, SynchronisedRegistryAcquireAccessError::VerificationFailure))
    )));
}

#[cfg(feature = "async")]
#[test]
fn async_acquire_view_reports_duplicate_resources() {
    use std::{
        future::Future,
        sync::Arc,
        task::{Context, Poll, Wake, Waker},
    };

    use crate::default::storages::registry_storage::ResourceView;

    struct NoopWake;
    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("duplicate");
    let mut future = std::pin::pin!(registry.acquire_view_async::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: None,
        resource_access: vec![(resource_id.clone(), None), (resource_id, None)],
        access: Access::Shared(1),
    }));
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);

    let result = match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("uncontended async lock did not resolve immediately"),
    };

    assert!(result.is_err_and(|error| matches!(
        error,
        SynchronisedRegistryAcquireViewError::DuplicateIds(indices) if indices == vec![1]
    )));
}

#[test]
fn cannot_acquire_view_with_missing_resource() {
    use crate::default::storages::registry_storage::ResourceView;

    let registry = create_registry(None);

    let resource_id1 = ResourceId::new_label("1");
    let resource1 = Resource::new("resource1".to_string());

    let result = registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id1.clone(),
        resource: Some(resource1.clone()),
        password: None,
    });

    assert!(result.ok());

    let resource_id_missing = ResourceId::new_label("missing");

    let result = registry.acquire_view::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: None,
        resource_access: vec![
            (resource_id1.clone(), None),
            (resource_id_missing, None),
        ],
        access: Access::Shared(1),
    });

    assert!(result.is_err_and(|err| err == SynchronisedRegistryAcquireViewError::NotFound));
}

#[cfg(all(feature = "notifier", feature = "releaser"))]
#[test]
fn released_view_deaccesses_and_notifies_on_drop() {
    use std::{future::Future, pin::Pin, sync::Arc, task::{Context, Poll}};

    use crate::default::storages::registry_storage::ResourceView;
    use aion_state::prelude::{AccessFilter, FutureAcquireAccess, RegistryOwnedAcquireAccess, RegistryReleaseAccess};

    struct RetryAcquire;
    impl AccessFilter for RetryAcquire {
        type Error = aion_state::prelude::SynchronisedRegistryAcquireAccessError;

        fn retry(&self, _error: &Self::Error) -> bool {
            true
        }
    }

    fn future_for<'a>(registry: &'a Arc<crate::TestRegistry>, resource_id: &ResourceId)
        -> Pin<Box<dyn Future<Output = Result<AccessResult<crate::default::storages::registry_storage::ResourceWrapper<'a, Resource>>, aion_state::prelude::SynchronisedRegistryAcquireAccessError>> + 'a>>
    {
        Box::pin(FutureAcquireAccess::<
            crate::default::storages::registry_storage::ResourceWrapper<'a, Resource>,
            crate::TestRegistry,
            RetryAcquire,
            ReserverId,
            Password,
            ResourceId,
            Access,
            Password,
            AccessResult<crate::default::storages::registry_storage::ResourceWrapper<'a, Resource>>,
        >::new(
            registry.as_ref(),
            RegistryOwnedAcquireAccess {
                user_details: None,
                resource_id: resource_id.clone(),
                access: Access::Unique,
                password: None,
            },
            RetryAcquire,
        ))
    }

    let registry = Arc::new(create_registry(None));
    let resource_ids = [
        ResourceId::new_label("released-view-1"),
        ResourceId::new_label("released-view-2"),
    ];
    let mut futures = resource_ids.iter().map(|resource_id| {
        assert!(registry.checked_replace(RegistryReplacement {
            user_details: None,
            access: &Access::Replace,
            resource_id: resource_id.clone(),
            resource: Some(Resource::new("resource".to_string())),
            password: None,
        }).ok());

        future_for(&registry, resource_id)
    }).collect::<Vec<_>>();

    let (waker, wake_count) = crate::feature_tests::counting_waker();
    let mut context = Context::from_waker(&waker);
    for future in &mut futures {
        assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));
    }

    {
        let _view = registry.acquire_released_view::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryOwnedAcquireView {
            user_details: None,
            resource_access: resource_ids.iter().cloned().map(|resource_id| (resource_id, None)).collect(),
            access: Access::Shared(1),
        }).unwrap();
    }
    assert!(wake_count.count() >= resource_ids.len(), "dropping the view should wake each resource waiter");

    let mut unique = Vec::with_capacity(futures.len());
    for future in &mut futures {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(Ok(acquired)) => unique.push(acquired),
            Poll::Ready(Err(error)) => panic!("notified unique access failed: {error:?}"),
            Poll::Pending => panic!("view release did not wake an access future"),
        }
    }
    drop(futures);
    drop(unique);
    for resource_id in &resource_ids {
        assert!(matches!(unsafe { registry.release_access(&RegistryReleaseAccess {
            resource_id,
            access: &Access::Unique,
        }) }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
    }
}

#[cfg(all(feature = "async", feature = "releaser"))]
#[test]
fn async_released_view_deaccesses_on_drop() {
    use std::{
        future::Future,
        sync::Arc,
        task::{Context, Poll, Wake, Waker},
    };

    use crate::default::storages::registry_storage::ResourceView;

    struct NoopWake;
    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    let registry = Arc::new(create_registry(None));
    let resource_id = ResourceId::new_label("async-released-view");
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    }).ok());

    let mut future = std::pin::pin!(registry.async_acquire_released_view::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryOwnedAcquireView {
        user_details: None,
        resource_access: vec![(resource_id.clone(), None)],
        access: Access::Shared(1),
    }));
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let view = match future.as_mut().poll(&mut context) {
        Poll::Ready(Ok(view)) => view,
        Poll::Ready(Err(error)) => panic!("async view acquisition failed: {error:?}"),
        Poll::Pending => panic!("uncontended async lock did not resolve immediately"),
    };
    assert!(matches!(view.as_ref(), Some(AccessResult::Shared(ResourceView::Shared(_)))));
    drop(view);

    let unique = registry.acquire_access::<AccessResult<crate::default::storages::registry_storage::ResourceWrapper<'_, Resource>>>(aion_state::prelude::RegistryAcquireAccess {
        user_details: None,
        resource_id,
        access: Access::Unique,
        password: None,
    });
    assert!(matches!(unique, Ok(AccessResult::Unique(_))));
}
