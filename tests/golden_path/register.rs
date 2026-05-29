use aion_state::{default::prelude::{Password, ReserverId}, prelude::RegistryRegister};

use crate::create_registry;

fn can_register() {
    let registry = create_registry();

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());
}

#[cfg(not(feature = "loom"))]
#[test]
fn can_register_normal() {
    can_register()
}