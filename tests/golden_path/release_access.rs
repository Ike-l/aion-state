#![cfg(feature = "notifier")]

use aion_state::prelude::{ReceptionGetAccess, RegistryAcquireAccess, RegistryReleaseAccess, SynchronisedRegistryReleaseAccessResult};
use crate::default::prelude::*;
use crate::default::storages::registry_storage::ResourceWrapper;
use super::owned_registry;

#[test]
fn release_access_clears_the_recorded_access() {
    let (registry, id, password, resource_id) = owned_registry();
    let access = Access::Shared(1);
    let acquired = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: Some((&id, &password)), resource_id: resource_id.clone(),
        access: access.clone(), password: None,
    }).unwrap();
    assert!(matches!(acquired, AccessResult::Shared(_)));
    drop(acquired);

    let result = unsafe { registry.release_access(&RegistryReleaseAccess { resource_id: &resource_id, access: &access }) };
    assert!(matches!(result, SynchronisedRegistryReleaseAccessResult::Ok));
    assert_eq!(registry.get_access(&ReceptionGetAccess { access_id: &resource_id }), Some(Access::Shared(0)));
}
