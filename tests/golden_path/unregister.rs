use aion_state::prelude::{RegistryRegister, RegistryUnregister};

use crate::default::prelude::*;
use crate::create_registry;

fn can_unregister() {
    let registry = create_registry(None);

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

#[test]
fn can_unregister_normal() {
    can_unregister();
}