use aion_state::{default::prelude::{Password, ReserverId}, prelude::{RegistryRegister, RegistryUnregister, RegistryUpdatePassword}};

use crate::create_registry;

fn can_update_password() {
    let registry = create_registry();

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());

    let result = registry.update_password(RegistryUpdatePassword {
        id: &id,
        old_password: &password,
        new_password: Password::from(2)
    });
    
    assert!(result.ok());

    let result = registry.unregister(&RegistryUnregister {
        id: &id,
        password: &Password::from(2)
    });

    assert!(result.ok());
}

#[test]
fn can_update_password_normal() {
    can_update_password();
}