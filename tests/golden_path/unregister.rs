use aion_state::{default::prelude::{Password, ReserverId}, prelude::{RegistryRegister, RegistryUnregister}};

use crate::create_registry;

#[test]
pub fn can_unregister() {
    let registry = create_registry();

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());

    let result = registry.unregister(&RegistryUnregister {
        id: &id, 
        password: &password
    });

    assert!(result.ok())
}