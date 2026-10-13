use aion_state::prelude::{RegistryRegister, SynchronisedRegistryRegisterResult};

use crate::default::prelude::*;
use crate::create_registry;

fn can_register() {
    let registry = create_registry(None);

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(matches!(result, SynchronisedRegistryRegisterResult::Ok));
    assert_eq!(registry.registered(), vec![id.clone()]);
    assert!(matches!(registry.register(RegistryRegister { id: id.clone(), password }), SynchronisedRegistryRegisterResult::Err));
    assert_eq!(registry.registered(), vec![id]);
}

#[test]
fn can_register_normal() {
    can_register()
}
