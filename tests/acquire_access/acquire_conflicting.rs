use aion_state::prelude::{RegistryAcquireAccess, SynchronisedRegistryAcquireAccessError, RegistryReplacement};

use crate::default::prelude::*;

use crate::create_registry;

#[test]
fn can_acquire_conflicting_u() {
    let registry = create_registry();

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
        AccessResult::Unique(resource_result) => *resource_result == resource,
        _ => false
    });

    let result = registry.acquire_access::<AccessResult<'_, Resource>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Unique,
        password: None
    });

    assert!(result.is_err_and(|err| err == SynchronisedRegistryAcquireAccessError::AccessConflict));
}

#[test]
fn can_acquire_conflicting_s() {
    let registry = create_registry();

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
        AccessResult::Shared(resource_result) => *resource_result == resource,
        _ => false
    });

    let result = registry.acquire_access::<AccessResult<'_, Resource>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Unique,
        password: None
    });

    assert!(result.is_err_and(|err| err == SynchronisedRegistryAcquireAccessError::AccessConflict));
}

#[test]
fn can_acquire_conflicting_uu() {
    let registry = create_registry();

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
        AccessResult::Unique(resource_result) => *resource_result == resource,
        _ => false
    });

    let result = registry.acquire_access::<AccessResult<'_, Resource>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Unique,
        password: None
    });

    assert!(result.is_err_and(|err| err == SynchronisedRegistryAcquireAccessError::AccessConflict));
}