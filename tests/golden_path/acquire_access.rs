use aion_state::prelude::{RegistryAcquireAccess, RegistrySaferReplacement};

use crate::default::prelude::*;

use crate::create_registry;

fn can_acquire_access() {
    let registry = create_registry();

    let resource_id = ResourceId::new_type::<String>();
    let resource = Resource::new("resource".to_string());

    let result = registry.safer_replace(RegistrySaferReplacement {
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
        AccessResult::Shared(resource_result) if *resource_result == resource => true,
        _ => false
    })

}

#[test]
fn can_acquire_access_normal() {
    can_acquire_access();
}