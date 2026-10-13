use aion_state::prelude::{RegistryAcquireAccess, SynchronisedRegistryAcquireAccessError, RegistryReplacement};

use crate::default::prelude::*;

use crate::create_registry;

fn registry_with_resource() -> (crate::TestRegistry, ResourceId, Resource) {
    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("conflicting-access");
    let resource = Resource::new("resource".to_string());
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(resource.clone()),
        password: None,
    }).ok());
    (registry, resource_id, resource)
}

#[test]
fn can_acquire_conflicting_u() {
    use crate::default::storages::registry_storage::ResourceWrapper;
    let (registry, resource_id, resource) = registry_with_resource();

    let result = registry.acquire_access(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Unique,
        password: None
    }).unwrap();

    assert!(match result {
        AccessResult::Unique(ResourceWrapper::Unique(resource_result)) => *resource_result == resource,
        _ => false
    });

    let result = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Unique,
        password: None
    });

    assert!(result.is_err_and(|err| err == SynchronisedRegistryAcquireAccessError::AccessConflict));
}

#[test]
fn can_acquire_conflicting_s() {
    use crate::default::storages::registry_storage::ResourceWrapper;
    let (registry, resource_id, resource) = registry_with_resource();

    let result = registry.acquire_access(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None
    }).unwrap();

    assert!(match result {
        AccessResult::Shared(ResourceWrapper::Shared(resource_result)) => *resource_result == resource,
        _ => false
    });

    let result = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Unique,
        password: None
    });

    assert!(result.is_err_and(|err| err == SynchronisedRegistryAcquireAccessError::AccessConflict));
}

#[test]
fn unique_access_conflicts_with_a_shared_request() {
    use crate::default::storages::registry_storage::ResourceWrapper;
    let (registry, resource_id, resource) = registry_with_resource();

    let result = registry.acquire_access(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Unique,
        password: None
    }).unwrap();

    assert!(match result {
        AccessResult::Unique(ResourceWrapper::Unique(resource_result)) => *resource_result == resource,
        _ => false
    });

    let result = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None
    });

    assert!(result.is_err_and(|err| err == SynchronisedRegistryAcquireAccessError::AccessConflict));
}
