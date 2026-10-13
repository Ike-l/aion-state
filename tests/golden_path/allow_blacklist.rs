use aion_state::prelude::{RegistryAllow, RegistryCheckAccess, RegistryRegister, SynchronisedRegistryBlacklistAllowResult, SynchronisedRegistryCheckAccessResult};
use crate::default::prelude::*;
use super::owned_registry;

#[test]
fn blacklist_allow_requires_its_generated_password() {
    let (registry, owner, owner_password, resource_id) = owned_registry();
    let guest = ReserverId::new("blacklist-guest");
    let guest_password = Password::from(2);
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: guest_password.clone() }).ok());

    let blacklist_password = match registry.allow_blacklist(RegistryAllow {
        id: &owner,
        password: &owner_password,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
    }) {
        SynchronisedRegistryBlacklistAllowResult::Ok(password) => password,
        result => panic!("expected blacklist entry, got {result:?}"),
    };

    let check = |password| RegistryCheckAccess {
        user_details: Some((&guest, &guest_password)),
        resource_id: &resource_id,
        access: &Access::Shared(1),
        password,
    };
    assert!(matches!(registry.check_access(&check(None)), SynchronisedRegistryCheckAccessResult::ListsDenied));
    let incorrect_password = [0u64, 1].into_iter().map(Password::new).find(|candidate| candidate != &blacklist_password).unwrap();
    assert!(matches!(registry.check_access(&check(Some(&incorrect_password))), SynchronisedRegistryCheckAccessResult::ListsDenied));
    assert!(matches!(registry.check_access(&check(Some(&blacklist_password))), SynchronisedRegistryCheckAccessResult::ContainsResource));
    assert!(matches!(registry.allow_blacklist(RegistryAllow {
        id: &guest,
        password: &guest_password,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
    }), SynchronisedRegistryBlacklistAllowResult::OwnershipDenied));
    assert!(matches!(registry.allow_blacklist(RegistryAllow {
        id: &owner,
        password: &Password::from(0),
        resource_id,
        access: Access::Unique,
    }), SynchronisedRegistryBlacklistAllowResult::VerificationFailure));
}
