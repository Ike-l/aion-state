mod register;
mod unregister;
mod update_password;
mod own;
mod release_resource;
mod release_resource_all;
mod allow_blacklist;
mod allow_whitelist;
mod unallow_blacklist;
mod unallow_whitelist;
mod check_access;
mod release_access;
mod reserve;
mod unreserve;
mod drain_reservations;
mod safer_replace;
mod contains_resource;
mod get_access;
mod default_registry;

use aion_state::prelude::{RegistryOwn, RegistryRegister, RegistryReplacement};
use crate::default::prelude::*;

pub(super) fn owned_registry() -> (crate::TestRegistry, ReserverId, Password, ResourceId) {
    let registry = crate::create_registry(None);
    let id = ReserverId::new("golden-user");
    let password = Password::from(1);
    let resource_id = ResourceId::new_label("golden-resource");

    assert!(registry.register(RegistryRegister { id: id.clone(), password: password.clone() }).ok());
    assert!(registry.own(RegistryOwn {
        id: id.clone(),
        password: &password,
        resource_id: resource_id.clone(),
    }).ok());
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: Some((&id, &password)),
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("golden-value".to_string())),
        password: None,
    }).ok());

    (registry, id, password, resource_id)
}
