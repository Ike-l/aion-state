use aion_state::prelude::{
    RegistryAcquireAccess, RegistryAllow, RegistryOwn, RegistryRegister, RegistryReplacement,
    RegistryReservation, RegistryUnreserve, SynchronisedRegistryCheckedReplacementResult,
    SynchronisedRegistryBlacklistAllowResult,
    SynchronisedRegistryReallocatingReplacementResult, SynchronisedRegistryReservationResult,
    SynchronisedRegistryUnreserveResult,
};

use crate::{BruteStoredResource, create_brute_registry, default::prelude::*};
use crate::default::storages::registry_storage::ResourceWrapper;

#[test]
fn stable_deref_storage_allows_reallocating_replacement() {
    let registry = create_brute_registry(None);
    let resource_id = ResourceId::new_label("feature-stable-deref");

    assert!(registry.reallocating_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("before".to_string())),
        password: None,
    }).ok());

    let result = registry.reallocating_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id,
        resource: Some(Resource::new("after".to_string())),
        password: None,
    });
    assert!(matches!(result, SynchronisedRegistryReallocatingReplacementResult::Found(old) if old == Resource::new("before".to_string())));
}

#[test]
fn stable_deref_replacement_rejects_non_replace_access_without_mutating() {
    let registry = create_brute_registry(None);
    let resource_id = ResourceId::new_label("stable-deref-denied-access");
    assert!(registry.reallocating_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("before".to_string())),
        password: None,
    }).ok());

    assert!(matches!(registry.reallocating_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Shared(1),
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("denied".to_string())),
        password: None,
    }), SynchronisedRegistryReallocatingReplacementResult::DeniedAccess));
    assert!(matches!(registry.reallocating_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id,
        resource: Some(Resource::new("after".to_string())),
        password: None,
    }), SynchronisedRegistryReallocatingReplacementResult::Found(previous)
        if previous == Resource::new("before".to_string())));
}

#[test]
fn stable_deref_replacements_reject_active_access_conflicts() {
    let registry = create_brute_registry(None);
    let resource_id = ResourceId::new_label("stable-deref-access-conflict");
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("before".to_string())),
        password: None,
    }).ok());
    let acquired = match registry.acquire_access::<AccessResult<ResourceWrapper<'_, BruteStoredResource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }) {
        Ok(acquired) => acquired,
        Err(_) => panic!("shared acquisition should succeed before the conflicting replacement"),
    };

    assert!(matches!(registry.reallocating_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("denied".to_string())),
        password: None,
    }), SynchronisedRegistryReallocatingReplacementResult::AccessConflict));
    assert!(matches!(registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id,
        resource: Some(Resource::new("denied".to_string())),
        password: None,
    }), SynchronisedRegistryCheckedReplacementResult::AccessConflict));
    drop(acquired);
}

#[test]
fn stable_deref_replacements_reject_other_reservers_reservations() {
    let registry = create_brute_registry(None);
    let owner = ReserverId::new("stable-deref-owner");
    let owner_password = Password::new(51);
    let guest = ReserverId::new("stable-deref-guest");
    let guest_password = Password::new(52);
    let resource_id = ResourceId::new_label("stable-deref-reservation-conflict");
    assert!(registry.register(RegistryRegister { id: owner.clone(), password: owner_password.clone() }).ok());
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: guest_password.clone() }).ok());
    assert!(registry.own(RegistryOwn {
        id: owner.clone(), password: &owner_password, resource_id: resource_id.clone(),
    }).ok());
    assert!(registry.checked_replace(RegistryReplacement {
        user_details: Some((&owner, &owner_password)),
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("before".to_string())),
        password: None,
    }).ok());
    assert!(matches!(registry.reserve(RegistryReservation {
        id: owner.clone(), id_password: &owner_password, resource_id: resource_id.clone(),
        access: Access::Shared(1), password: None,
    }), SynchronisedRegistryReservationResult::OkNew));
    let blacklist_password = match registry.allow_blacklist(RegistryAllow {
        id: &owner, password: &owner_password, resource_id: resource_id.clone(), access: Access::Replace,
    }) {
        SynchronisedRegistryBlacklistAllowResult::Ok(password) => password,
        result => panic!("expected blacklist entry, got {result:?}"),
    };
    let wrong_resource_password = [0u64, 1].into_iter().map(Password::new)
        .find(|candidate| candidate != &blacklist_password).unwrap();
    let replacement = |password| RegistryReplacement {
        user_details: Some((&guest, &guest_password)),
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("denied".to_string())),
        password,
    };
    assert!(matches!(registry.reallocating_replace(replacement(Some(&wrong_resource_password))), SynchronisedRegistryReallocatingReplacementResult::ListsDenied));
    assert!(matches!(registry.checked_replace(replacement(Some(&wrong_resource_password))), SynchronisedRegistryCheckedReplacementResult::ListsDenied));
    assert!(registry.allow_whitelist(RegistryAllow {
        id: &owner, password: &owner_password, resource_id: resource_id.clone(), access: Access::Replace,
    }).ok());

    assert!(matches!(registry.reallocating_replace(replacement(None)), SynchronisedRegistryReallocatingReplacementResult::ReservationConflict));
    assert!(matches!(registry.checked_replace(replacement(None)), SynchronisedRegistryCheckedReplacementResult::ReservationConflict));
    assert!(matches!(registry.unreserve(&RegistryUnreserve {
        id: &owner, password: &owner_password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryUnreserveResult::Ok));

    let previous = registry.reallocating_replace(RegistryReplacement {
        user_details: Some((&owner, &owner_password)),
        access: &Access::Replace,
        resource_id,
        resource: Some(Resource::new("after".to_string())),
        password: None,
    });
    assert!(matches!(previous, SynchronisedRegistryReallocatingReplacementResult::Found(value)
        if value == Resource::new("before".to_string())));
}
