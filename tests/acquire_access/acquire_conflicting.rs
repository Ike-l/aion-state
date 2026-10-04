use aion_state::prelude::{RegistryAcquireAccess, SynchronisedRegistryAcquireAccessError, RegistryReplacement};

use crate::default::prelude::*;

use crate::create_registry;

#[test]
fn can_acquire_conflicting_u() {
    use crate::default::storages::registry_storage::ResourceWrapper;
    let registry = create_registry(None);

    let resource_id = ResourceId::new_label("1");
    let resource = Resource::new("resource".to_string());

    let result = registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(resource.clone()),
        password: None,
    });

    assert!(result.ok());

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
    let registry = create_registry(None);

    let resource_id = ResourceId::new_label("1");
    let resource = Resource::new("resource".to_string());

    let result = registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(resource.clone()),
        password: None,
    });

    assert!(result.ok());

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
fn can_acquire_conflicting_uu() {
    use crate::default::storages::registry_storage::ResourceWrapper;
    let registry = create_registry(None);

    let resource_id = ResourceId::new_label("1");
    let resource = Resource::new("resource".to_string());

    let result = registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(resource.clone()),
        password: None,
    });

    assert!(result.ok());

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