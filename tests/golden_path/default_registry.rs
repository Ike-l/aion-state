use aion_state::prelude::{RegistryAcquireAccess, RegistryReplacement, SynchronisedRegistryCheckedReplacementResult};

use crate::default::prelude::*;
use crate::default::storages::registry_storage::ResourceWrapper;

#[test]
fn default_registry_initializes_and_stores_a_resource() {
    let registry = crate::TestRegistry::default();
    let resource_id = ResourceId::new_label("default-registry");
    let resource = Resource::new("default-value".to_string());

    assert_eq!(0, registry.len());
    assert!(matches!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(resource.clone()),
        password: None,
    }), SynchronisedRegistryCheckedReplacementResult::NotFound));
    assert_eq!(1, registry.len());

    let acquired = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id,
        access: Access::Shared(1),
        password: None,
    }).unwrap();
    assert!(matches!(acquired, AccessResult::Shared(ResourceWrapper::Shared(value)) if *value == resource));
}
