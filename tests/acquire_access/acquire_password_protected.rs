use aion_state::prelude::{RegistryAcquireAccess, RegistryAllow, RegistryOwn, RegistryRegister, RegistryReplacement, SynchronisedRegistryAcquireAccessError, SynchronisedRegistryBlacklistAllowResult};
use crate::default::prelude::*;
use crate::default::primitives::accesses::access_result::Transmutable;
use crate::default::storages::registry_storage::ResourceWrapper;
use crate::create_registry;

fn owned_resource() -> (crate::TestRegistry, ReserverId, Password, ResourceId, Resource) {
    let registry = create_registry(None);
    let owner = ReserverId::new("owner");
    let owner_password = Password::new(1);
    let resource_id = ResourceId::new_label("protected-resource");
    let resource = Resource::new("resource".to_string());
    assert!(registry.register(RegistryRegister { id: owner.clone(), password: owner_password.clone() }).ok());
    assert!(registry.own(RegistryOwn {
        id: owner.clone(), password: &owner_password, resource_id: resource_id.clone(),
    }).ok());
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: Some((&owner, &owner_password)), access: &Access::Replace,
        resource_id: resource_id.clone(), resource: Some(resource.clone()), password: None,
    }).ok());
    (registry, owner, owner_password, resource_id, resource)
}

fn register_guest(registry: &crate::TestRegistry) -> (ReserverId, Password) {
    let guest = ReserverId::new("guest");
    let password = Password::new(2);
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: password.clone() }).ok());
    (guest, password)
}

#[test]
fn whitelist_grants_only_the_listed_access_to_another_user() {
    let (registry, owner, owner_password, resource_id, mut resource) = owned_resource();
    let (guest, _) = register_guest(&registry);
    assert!(registry.allow_whitelist(RegistryAllow {
        id: &owner, password: &owner_password, resource_id: resource_id.clone(), access: Access::Shared(1),
    }).ok());

    assert_eq!(registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: Some((&guest, &Password::new(2))), resource_id: resource_id.clone(),
        access: Access::Unique, password: None,
    }).unwrap_err(), SynchronisedRegistryAcquireAccessError::ListsDenied);

    let acquired = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: Some((&guest, &Password::new(2))), resource_id: resource_id.clone(),
        access: Access::Shared(1), password: None,
    }).unwrap();
    assert_eq!(acquired, AccessResult::Shared(ResourceWrapper::new(&mut resource).transmute()));
}

#[test]
fn a_whitelist_can_grant_unique_access_independently() {
    let (registry, owner, owner_password, resource_id, mut resource) = owned_resource();
    let (guest, guest_password) = register_guest(&registry);
    assert!(registry.allow_whitelist(RegistryAllow {
        id: &owner, password: &owner_password, resource_id: resource_id.clone(), access: Access::Unique,
    }).ok());

    let acquired = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: Some((&guest, &guest_password)), resource_id,
        access: Access::Unique, password: None,
    }).unwrap();
    assert_eq!(acquired, AccessResult::Unique(ResourceWrapper::new(&mut resource)));
}

#[test]
fn blacklist_requires_the_password_returned_when_it_is_added() {
    let (registry, owner, owner_password, resource_id, mut resource) = owned_resource();
    let (guest, guest_password) = register_guest(&registry);
    let blacklist_password = match registry.allow_blacklist(RegistryAllow {
        id: &owner, password: &owner_password, resource_id: resource_id.clone(), access: Access::Shared(1),
    }) {
        SynchronisedRegistryBlacklistAllowResult::Ok(password) => password,
        result => panic!("expected blacklist entry, got {result:?}"),
    };

    let wrong_password = [0u64, 1].into_iter().map(Password::new).find(|candidate| candidate != &blacklist_password).unwrap();
    for supplied_password in [None, Some(&wrong_password)] {
        assert_eq!(registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
            user_details: Some((&guest, &guest_password)), resource_id: resource_id.clone(),
            access: Access::Shared(1), password: supplied_password,
        }).unwrap_err(), SynchronisedRegistryAcquireAccessError::ListsDenied);
    }

    let acquired = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: Some((&guest, &guest_password)), resource_id,
        access: Access::Shared(1), password: Some(&blacklist_password),
    }).unwrap();
    assert_eq!(acquired, AccessResult::Shared(ResourceWrapper::new(&mut resource).transmute()));
}
