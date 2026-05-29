use aion_state::{default::prelude::{Password, ReserverId, ResourceId}, prelude::{RegistryOwn, RegistryRegister}};

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