use aion_state::prelude::{RegistryAcquireAccess, SynchronisedRegistryAcquireAccessError, RegistryOwn, RegistryRegister, RegistrySaferReplacement};

use std::assert_matches;

use crate::default::prelude::*;
use crate::create_registry;

fn can_own() {
    let registry = create_registry();

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());

    let result = registry.own(RegistryOwn {
        id,
        password: &password,
        resource_id: ResourceId::new_type::<i32>()
    });

    assert!(result.ok())
}


#[test]
fn can_own_normal() {
    can_own();
}

fn owning_blocks_by_default() {
    let registry = create_registry();

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());

    let resource_id = ResourceId::new_type::<i32>();
    let result = registry.own(RegistryOwn {
        id: id.clone(),
        password: &password,
        resource_id: resource_id.clone()
    });

    assert!(result.ok());

    let result = registry.checked_replace(RegistrySaferReplacement {
        user_details: Some((&id, &password)),
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    });

    assert!(result.ok());

    let result = registry.acquire_access::<AccessResult<Resource>>(RegistryAcquireAccess {
        user_details: None,
        access: Access::Shared(1),
        resource_id,
        password: None,
    });

    match result {
        Ok(_) => panic!("Expected Err"),
        Err(err) => {
            assert_matches!(err, SynchronisedRegistryAcquireAccessError::ListsDenied);
        }
    }
}

#[test]
fn owning_blocks_by_default_normal() {
    owning_blocks_by_default();
}
