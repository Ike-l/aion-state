use aion_state::prelude::{RegistryAcquireAccess, ReceptionGetAccess};
use crate::default::prelude::*;
use crate::default::storages::registry_storage::ResourceWrapper;
use super::owned_registry;

#[test]
fn get_access_observes_active_access() {
    let (registry, id, password, resource_id) = owned_registry();
    let get_access = ReceptionGetAccess { access_id: &resource_id };
    assert_eq!(registry.get_access(&get_access), None);

    let acquired = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: Some((&id, &password)), resource_id: resource_id.clone(),
        access: Access::Shared(1), password: None,
    }).unwrap();
    assert!(matches!(acquired, AccessResult::Shared(_)));
    assert_eq!(registry.get_access(&get_access), Some(Access::Shared(1)));
}
