use aion_state::prelude::{RegistryAcquireAccess, RegistryAllow, RegistryOwn, RegistryRegister, RegistryReplacement};

use crate::default::prelude::*;

use crate::create_registry;

#[test]
fn can_serde_with_owned() {
    let registry = create_registry(None);

    let resource_id = ResourceId::new_label("Bar");
    let resource = Resource::new("baz".to_string());

    let id = ReserverId::new("Foo");
    let password = Password::new(69);

    assert!(registry.register(RegistryRegister {
        id: id.clone(),
        password: password.clone(),
    }).ok());

    assert!(registry.own(RegistryOwn {
        id: id.clone(),
        password: &password,
        resource_id: resource_id.clone(),
    }).ok());

    assert!(registry.allow_whitelist(RegistryAllow { 
        id: &id, 
        password: &password, 
        resource_id: resource_id.clone(), 
        access: Access::Shared(1) 
    }).ok());

    assert!(registry.checked_replace(RegistryReplacement {
        user_details: Some((&id, &password)),
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(resource.clone()),
        password: None,
    }).ok());

    let result = registry.acquire_access::<AccessResult<'_, Resource>>(RegistryAcquireAccess {
        user_details: Some((&id, &password)),
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None
    });

    assert_eq!(result, Ok(AccessResult::Shared(&resource)));

    let s = serde_json::to_string(&registry).unwrap();
    panic!("S: {s}");
}