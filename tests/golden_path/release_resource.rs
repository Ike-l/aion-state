use aion_state::{default::prelude::{Password, ReserverId, ResourceId}, prelude::{RegistryOwn, RegistryRegister, RegistryReleaseResource}};

use crate::create_registry;

fn can_release_resource() {
    let registry = create_registry();

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());

    let resource_id = ResourceId::new_type::<i32>();
    let result = registry.own(RegistryOwn {
        id: id.clone(),
        password: &password,
        resource_id: resource_id.clone()
    });

    assert!(result.ok());

    let result = registry.release_resource(&RegistryReleaseResource {
        id: &id,
        password: &password,
        resource_id: &resource_id
    });

    assert!(result.ok())
}

#[cfg(not(feature = "loom"))]
#[test]
fn can_release_resource_normal() {
    can_release_resource();
}
