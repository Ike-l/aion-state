use aion_state::prelude::RegistryRegister;

use crate::default::prelude::*;
use crate::create_registry;

fn can_register() {
    let registry = create_registry(None);

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());
}

#[test]
fn can_register_normal() {
    can_register()
}