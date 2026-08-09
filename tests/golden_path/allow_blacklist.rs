use aion_state::prelude::{RegistryAllow, RegistryOwn, RegistryRegister};
use crate::default::prelude::*;

use crate::create_registry;

fn can_allow_blacklist() {
    let registry = create_registry(None);

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister {
        id: id.clone(),
        password: password.clone(),
    });

    assert!(result.ok());

    let resource_id = ResourceId::new_type::<String>();
    let resource_access = Access::Shared(1);
    
    let result = registry.own(RegistryOwn {
        id: id.clone(),
        password: &password,
        resource_id: resource_id.clone()
    });

    assert!(result.ok());

    let result = registry.allow_blacklist(RegistryAllow {
        id: &id,
        password: &password,
        resource_id: resource_id.clone(),
        access: resource_access.clone()
    });

    assert!(result.ok());
}

#[test]
fn can_allow_blacklist_normal() {
    can_allow_blacklist();
}