use aion_state::prelude::{RegistryAcquireAccess, SynchronisedRegistryAcquireAccessError, RegistryOwn, RegistryRegister, RegistryReplacement};

use crate::default::prelude::*;

use crate::create_registry;

#[test]
fn cant_acquire_owned() {
    let registry = create_registry();

    let resource_id = ResourceId::new_type::<String>();
    let resource = Resource::new("resource".to_string());

    let id = ReserverId::new("1");
    let password = Password::new(1);

    assert!(registry.register(RegistryRegister {
        id: id.clone(),
        password: password.clone(),
    }).ok());

    assert!(registry.own(RegistryOwn {
        id: id.clone(),
        password: &password,
        resource_id: resource_id.clone(),
    }).ok());

    assert!(registry.checked_replace(RegistryReplacement {
        user_details: Some((&id, &password)),
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(resource.clone()),
        password: None,
    }).ok());

    let result = registry.acquire_access::<AccessResult<'_, Resource>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None
    });

    assert_eq!(result, Err(SynchronisedRegistryAcquireAccessError::ListsDenied));
}

#[test]
fn can_acquire_owned() {
    let registry = create_registry();

    let resource_id = ResourceId::new_type::<String>();
    let resource = Resource::new("resource".to_string());

    let id = ReserverId::new("1");
    let password = Password::new(1);

    assert!(registry.register(RegistryRegister {
        id: id.clone(),
        password: password.clone(),
    }).ok());

    assert!(registry.own(RegistryOwn {
        id: id.clone(),
        password: &password,
        resource_id: resource_id.clone(),
    }).ok());

    assert!(registry.checked_replace(RegistryReplacement {
        user_details: Some((&id, &password)),
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(resource.clone()),
        password: None,
    }).ok());

    let result = registry.acquire_access::<AccessResult<'_, Resource>>(RegistryAcquireAccess {
        user_details: Some((&id, &password)),
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None
    });

    assert_eq!(result, Ok(AccessResult::Shared(&resource)));
}