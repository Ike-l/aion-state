use aion_state::{default::prelude::{Password, ReserverId}, prelude::RegistryRegister};
use tracing::{Level, event, span};

use crate::create_registry;

#[test]
pub fn can_register() {
    let registry = create_registry();


    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let span = span!(Level::TRACE, "can register");
    let _enter = span.enter();

    event!(Level::TRACE, "A");

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());
}