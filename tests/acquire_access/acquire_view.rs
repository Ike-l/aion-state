use aion_state::prelude::{RegistryAcquireView, SynchronisedRegistryAcquireViewError, RegistryReplacement};

use crate::default::prelude::*;

use crate::create_registry;

#[test]
fn can_acquire_view_shared() {
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

    let resource_id2 = ResourceId::new_label("2");
    let resource2 = Resource::new("resource2".to_string());

    let result = registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id2.clone(),
        resource: Some(resource2.clone()),
        password: None,
    });

    assert!(result.ok());

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

    let resource_id2 = ResourceId::new_label("2");
    let resource2 = Resource::new("resource2".to_string());

    let result = registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id2.clone(),
        resource: Some(resource2.clone()),
        password: None,
    });

    assert!(result.ok());

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
