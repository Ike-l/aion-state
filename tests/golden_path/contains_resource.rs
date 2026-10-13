use aion_state::prelude::{RegistryContainsResource, RegistryReplacement, SynchronisedRegistryContainsResourceResult};
use crate::default::prelude::*;
use super::owned_registry;

#[test]
fn contains_resource_tracks_insert_and_remove() {
    let (registry, id, password, resource_id) = owned_registry();
    let contains = |resource_id: &ResourceId| registry.contains_resource(&RegistryContainsResource { resource_id });

    assert!(matches!(contains(&resource_id), SynchronisedRegistryContainsResourceResult::Some));
    let missing_id = ResourceId::new_label("missing-resource");
    assert!(matches!(contains(&missing_id), SynchronisedRegistryContainsResourceResult::None));
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: Some((&id, &password)),
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: None,
        password: None,
    }).ok());
    assert!(matches!(contains(&resource_id), SynchronisedRegistryContainsResourceResult::None));
}
