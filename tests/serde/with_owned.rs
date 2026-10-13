use aion_state::prelude::{RegistryAcquireAccess, RegistryCheckOwner, RegistryIsOwned, RegistryOwn, RegistryRegister, RegistryReplacement};
use crate::default::prelude::*;
use crate::default::storages::registry_storage::ResourceWrapper;
use crate::create_registry;

#[test]
fn serde_round_trip_preserves_owned_registry_state() {
    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("serialized-resource");
    let id = ReserverId::new("serialized-owner");
    let password = Password::new(69);
    let resource = Resource::new("serialized-value".to_string());

    assert!(registry.register(RegistryRegister { id: id.clone(), password: password.clone() }).ok());
    assert!(registry.own(RegistryOwn {
        id: id.clone(), password: &password, resource_id: resource_id.clone(),
    }).ok());
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: Some((&id, &password)), access: &Access::Replace,
        resource_id: resource_id.clone(), resource: Some(resource.clone()), password: None,
    }).ok());

    let json = serde_json::to_string(&registry).unwrap();
    let restored: crate::TestRegistry = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.len(), 1);
    assert_eq!(restored.registered(), vec![id.clone()]);
    assert!(restored.is_owned(&RegistryIsOwned { resource_id: &resource_id }));
    assert!(restored.check_owner(&RegistryCheckOwner { id: &id, resource_id: &resource_id }).ok());

    let access = restored.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: Some((&id, &password)), resource_id, access: Access::Unique, password: None,
    }).unwrap();
    let AccessResult::Unique(ResourceWrapper::Unique(restored_resource)) = access else {
        panic!("restored resource should be returned through unique access");
    };
    assert_eq!(*restored_resource, resource);
}

#[test]
fn serde_reports_invalid_registry_and_reception_state() {
    let value = serde_json::to_value(crate::TestRegistry::default()).unwrap();

    let mut invalid_registry = value.clone();
    invalid_registry["unsynchronised_registry"]["automated_registry"] = serde_json::Value::Null;
    assert!(serde_json::from_value::<crate::TestRegistry>(invalid_registry).is_err());

    let mut invalid_reception = value;
    invalid_reception["unsynchronised_registry"]["reception"] = serde_json::Value::Null;
    assert!(serde_json::from_value::<crate::TestRegistry>(invalid_reception).is_err());
}
