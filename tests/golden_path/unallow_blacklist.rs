use aion_state::prelude::{RegistryAllow, RegistryCheckAccess, RegistryRegister, RegistryUnallow, SynchronisedRegistryBlacklistAllowResult, SynchronisedRegistryBlacklistUnallowResult, SynchronisedRegistryCheckAccessResult};
use crate::default::prelude::*;
use super::owned_registry;

#[test]
fn unallow_blacklist_removes_password_requirement_and_reports_failures() {
    let (registry, owner, owner_password, resource_id) = owned_registry();
    let guest = ReserverId::new("blacklist-guest");
    let guest_password = Password::from(2);
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: guest_password.clone() }).ok());
    let blacklist_password = match registry.allow_blacklist(RegistryAllow {
        id: &owner, password: &owner_password, resource_id: resource_id.clone(), access: Access::Shared(1),
    }) {
        SynchronisedRegistryBlacklistAllowResult::Ok(password) => password,
        result => panic!("expected blacklist entry, got {result:?}"),
    };
    let check = RegistryCheckAccess {
        user_details: Some((&guest, &guest_password)), resource_id: &resource_id,
        access: &Access::Shared(1), password: Some(&blacklist_password),
    };
    assert!(matches!(registry.check_access(&check), SynchronisedRegistryCheckAccessResult::ContainsResource));
    assert!(matches!(registry.unallow_blacklist(&RegistryUnallow {
        id: &owner, password: &owner_password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryBlacklistUnallowResult::Ok));
    assert!(matches!(registry.check_access(&check), SynchronisedRegistryCheckAccessResult::ListsDenied));
    assert!(matches!(registry.unallow_blacklist(&RegistryUnallow {
        id: &owner, password: &owner_password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryBlacklistUnallowResult::Err));
    assert!(matches!(registry.unallow_blacklist(&RegistryUnallow {
        id: &owner, password: &Password::from(9), resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryBlacklistUnallowResult::VerificationFailure));
    assert!(matches!(registry.unallow_blacklist(&RegistryUnallow {
        id: &guest, password: &guest_password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryBlacklistUnallowResult::OwnershipDenied));
}
