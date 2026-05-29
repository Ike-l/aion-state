use aion_state::{default::prelude::{Access, Password, ReserverId, ResourceId}, prelude::{RegistryAcquireAccess, RegistryAcquireAccessError, RegistryOwn, RegistryRegister}};

use std::assert_matches;

use crate::create_registry;

#[test]
pub fn can_own() {
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
fn owning_blocks_by_default() {
    let registry = create_registry();

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());

    let resource_id = ResourceId::new_type::<i32>();
    let result = registry.own(RegistryOwn {
        id,
        password: &password,
        resource_id: resource_id.clone()
    });

    assert!(result.ok());

    let result = registry.acquire_access(RegistryAcquireAccess {
        user_details: None,
        access: Access::Shared(1),
        resource_id,
        password: None,
    });

    match result {
        Ok(_) => panic!("Expected Err"),
        Err(err) => {
            assert_matches!(err, RegistryAcquireAccessError::WhitelistDenied);
        }
    }
}