use std::sync::Arc;

use aion_state::prelude::{
    RegistryAcquireAccess, RegistryAcquireView, RegistryOwn, RegistryOwnedAcquireAccess, RegistryReplacement,
    RegistryOwnedAcquireView, RegistryRegister, ReceptionGetAccess, Releaser,
    SynchronisedRegistryAcquireAccessError, ViewReleaser,
};

use crate::{TestRegistry, create_registry, default::prelude::*};

fn insert_resource(registry: &TestRegistry, resource_id: ResourceId) {
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id,
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    }).ok());
}

#[test]
fn releaser_access_guard_releases_when_dropped() {
    let registry = Arc::new(create_registry(None));
    let resource_id = ResourceId::new_label("feature-releaser-access");
    insert_resource(&registry, resource_id.clone());

    {
        let guard = registry.acquire_released_access::<AccessResult<
            crate::default::storages::registry_storage::ResourceWrapper<'_, Resource>,
        >>(RegistryOwnedAcquireAccess {
            user_details: None,
            resource_id: resource_id.clone(),
            access: Access::Shared(1),
            password: None,
        }).unwrap();
        assert!(guard.as_ref().is_some());
    }

    assert_eq!(Some(Access::Shared(0)), registry.get_access(&ReceptionGetAccess { access_id: &resource_id }));
    assert!(registry.acquire_access::<AccessResult<
        crate::default::storages::registry_storage::ResourceWrapper<'_, Resource>,
    >>(RegistryAcquireAccess {
        user_details: None,
        resource_id,
        access: Access::Unique,
        password: None,
    }).is_ok());
}

#[test]
fn releaser_view_guard_releases_every_id_when_dropped() {
    let registry = Arc::new(create_registry(None));
    let resource_ids = [
        ResourceId::new_label("feature-releaser-view-a"),
        ResourceId::new_label("feature-releaser-view-b"),
    ];
    for resource_id in &resource_ids {
        insert_resource(&registry, resource_id.clone());
    }

    {
        let guard = registry.acquire_released_view::<AccessResult<
            crate::default::storages::registry_storage::ResourceView<'_, ResourceId, Resource>,
        >>(RegistryOwnedAcquireView {
            user_details: None,
            resource_access: resource_ids.iter().cloned().map(|id| (id, None)).collect(),
            access: Access::Shared(1),
        }).unwrap();
        assert!(guard.as_ref().is_some());
    }

    for resource_id in &resource_ids {
        assert_eq!(Some(Access::Shared(0)), registry.get_access(&ReceptionGetAccess { access_id: resource_id }));
    }
    assert!(registry.acquire_view::<AccessResult<
        crate::default::storages::registry_storage::ResourceView<'_, ResourceId, Resource>,
    >>(RegistryAcquireView {
        user_details: None,
        resource_access: resource_ids.iter().cloned().map(|id| (id, None)).collect(),
        access: Access::Unique,
    }).is_ok());
}

#[test]
fn releasing_result_accessors_and_update_keep_the_release_token() {
    let registry = Arc::new(create_registry(None));
    let resource_id = ResourceId::new_label("releaser-update");
    insert_resource(&registry, resource_id.clone());
    let mut guard = registry.acquire_released_access::<AccessResult<
        crate::default::storages::registry_storage::ResourceWrapper<'_, Resource>,
    >>(RegistryOwnedAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }).unwrap();

    assert!(guard.as_ref().is_some());
    assert!(guard.as_mut().is_some());
    assert!(Arc::ptr_eq(guard.access_releaser().unwrap(), &registry));
    let release = guard.access_release_input().unwrap();
    assert_eq!(vec![resource_id.clone()], release.resource_ids);
    assert_eq!(Access::Shared(1), release.access);

    let updated = guard.update(|access_result| access_result);
    assert!(updated.as_ref().is_some());
    drop(updated);
    assert_eq!(Some(Access::Shared(0)), registry.get_access(&ReceptionGetAccess { access_id: &resource_id }));
}

#[test]
fn releaser_preserves_acquisition_errors() {
    let registry = Arc::new(create_registry(None));
    let missing = ResourceId::new_label("releaser-missing");
    let result = registry.acquire_released_access::<AccessResult<
        crate::default::storages::registry_storage::ResourceWrapper<'_, Resource>,
    >>(RegistryOwnedAcquireAccess {
        user_details: None,
        resource_id: missing,
        access: Access::Shared(1),
        password: None,
    });
    assert!(matches!(result, Err(SynchronisedRegistryAcquireAccessError::NotFound)));
}

#[test]
fn view_releaser_preserves_missing_resource_errors() {
    use aion_state::prelude::SynchronisedRegistryAcquireViewError;

    let registry = Arc::new(create_registry(None));
    let missing = ResourceId::new_label("view-releaser-missing");
    let result = registry.acquire_released_view::<AccessResult<
        crate::default::storages::registry_storage::ResourceView<'_, ResourceId, Resource>,
    >>(RegistryOwnedAcquireView {
        user_details: None,
        resource_access: vec![(missing, None)],
        access: Access::Shared(1),
    });
    assert!(matches!(result, Err(SynchronisedRegistryAcquireViewError::NotFound)));
}

#[test]
fn releaser_guards_forward_authenticated_user_details() {
    let registry = create_registry(None);
    let id = ReserverId::new("releaser-owner");
    let password = Password::new(31);
    let resource_ids = [ResourceId::new_label("releaser-owner-a"), ResourceId::new_label("releaser-owner-b")];
    assert!(registry.register(RegistryRegister { id: id.clone(), password: password.clone() }).ok());
    for resource_id in &resource_ids {
        assert!(registry.own(RegistryOwn {
            id: id.clone(), password: &password, resource_id: resource_id.clone(),
        }).ok());
        assert!(registry.checked_replace(RegistryReplacement {
            user_details: Some((&id, &password)), access: &Access::Replace,
            resource_id: resource_id.clone(), resource: Some(Resource::new("owner-value".to_string())),
            password: None,
        }).ok());
    }
    let registry = Arc::new(registry);

    {
        let guard = registry.acquire_released_access::<AccessResult<
            crate::default::storages::registry_storage::ResourceWrapper<'_, Resource>,
        >>(RegistryOwnedAcquireAccess {
            user_details: Some((id.clone(), password.clone())), resource_id: resource_ids[0].clone(),
            access: Access::Shared(1), password: None,
        }).unwrap();
        assert!(guard.as_ref().is_some());
    }
    assert_eq!(Some(Access::Shared(0)), registry.get_access(&ReceptionGetAccess { access_id: &resource_ids[0] }));

    {
        let guard = registry.acquire_released_view::<AccessResult<
            crate::default::storages::registry_storage::ResourceView<'_, ResourceId, Resource>,
        >>(RegistryOwnedAcquireView {
            user_details: Some((id.clone(), password.clone())),
            resource_access: resource_ids.iter().cloned().map(|resource_id| (resource_id, None)).collect(),
            access: Access::Shared(1),
        }).unwrap();
        assert!(guard.as_ref().is_some());
    }
    for resource_id in &resource_ids {
        assert_eq!(Some(Access::Shared(0)), registry.get_access(&ReceptionGetAccess { access_id: resource_id }));
    }
}
