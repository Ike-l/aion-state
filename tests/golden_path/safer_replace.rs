use aion_state::prelude::{RegistryAcquireAccess, RegistryContainsResource, RegistrySaferReplacement, SynchronisedRegistryReallocatingReplacementResult};

use std::assert_matches;

use crate::default::prelude::*;
use crate::create_registry;

fn can_safer_replace() {
    let registry = create_registry();

    let result = registry.reallocating_replace(RegistrySaferReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: ResourceId::Label("Foo".to_string()),
        resource: Some(Resource::new("foo".to_string())),
        password: None,
    });

    assert!(result.ok());

    assert!(registry.contains_resource(&RegistryContainsResource { resource_id: &ResourceId::Label("Foo".to_string()) }).ok());
}


#[test]
fn can_safer_replace_normal() {
    can_safer_replace();
}

fn multi_safer_replace() {
    let registry = create_registry();

    let raw_resources = 0..;

    let n = 100;
    for raw_resource in raw_resources.take(n) {
        let resource_id = ResourceId::Label(raw_resource.to_string());
        let resource = Resource::new(raw_resource.to_string());
        let result = registry.reallocating_replace(RegistrySaferReplacement {
            user_details: None,
            access: &Access::Replace,
            resource_id: resource_id.clone(),
            resource: Some(resource.clone()),
            password: None,
        });
    
        assert_matches!(result, SynchronisedRegistryReallocatingReplacementResult::NotFound);

        let result = registry.acquire_access(RegistryAcquireAccess {
            user_details: None,
            resource_id: resource_id.clone(),
            access: Access::Unique,
            password: None
        }).unwrap();

        let AccessResult::Unique(current_resource) = result else { unreachable!() };
        assert_eq!(*current_resource, resource);

        assert!(registry.contains_resource(&RegistryContainsResource { resource_id: &resource_id }).ok());
    }

    assert_eq!(registry.len(), n)
}

#[test]
fn multi_safer_replace_normal() {
    multi_safer_replace();
}
