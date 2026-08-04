use aion_state::prelude::{RegistryAcquireAccess, RegistryReplacement};

use crate::default::prelude::*;

use crate::create_registry;

#[test]
fn can_acquire_many() {
    let registry = create_registry();

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

    let resource_id3 = ResourceId::new_label("3");
    let resource3 = Resource::new("resource3".to_string());

    let result = registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id3.clone(),
        resource: Some(resource3.clone()),
        password: None,
    });

    assert!(result.ok());

    let result = registry.acquire_access(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id1.clone(),
        access: Access::Shared(1),
        password: None
    }).unwrap();

    assert!(match result {
        AccessResult::Shared(resource_result) => *resource_result == resource1,
        _ => false
    });

    let result = registry.acquire_access(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id2.clone(),
        access: Access::Shared(1),
        password: None
    }).unwrap();

    assert!(match result {
        AccessResult::Shared(resource_result) => *resource_result == resource2,
        _ => false
    });

    let result = registry.acquire_access(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id3.clone(),
        access: Access::Shared(1),
        password: None
    }).unwrap();

    assert!(match result {
        AccessResult::Shared(resource_result) => *resource_result == resource3,
        _ => false
    });
}