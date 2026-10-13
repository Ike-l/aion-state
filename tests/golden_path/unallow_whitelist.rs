use aion_state::prelude::{RegistryAllow, RegistryCheckAccess, RegistryRegister, RegistryUnallow, SynchronisedRegistryCheckAccessResult, SynchronisedRegistryWhitelistUnallowResult};
use crate::default::prelude::*;
use super::owned_registry;

#[test]
fn unallow_whitelist_removes_entry_and_reports_failures() {
    let (registry, owner, owner_password, resource_id) = owned_registry();
    let guest = ReserverId::new("whitelist-guest");
    let guest_password = Password::from(2);
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: guest_password.clone() }).ok());
    assert!(registry.allow_whitelist(RegistryAllow {
        id: &owner, password: &owner_password, resource_id: resource_id.clone(), access: Access::Shared(1),
    }).ok());
    let check = RegistryCheckAccess {
        user_details: Some((&guest, &guest_password)), resource_id: &resource_id,
        access: &Access::Shared(1), password: None,
    };
    assert!(matches!(registry.check_access(&check), SynchronisedRegistryCheckAccessResult::ContainsResource));
    assert!(matches!(registry.unallow_whitelist(&RegistryUnallow {
        id: &owner, password: &owner_password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryWhitelistUnallowResult::Ok));
    assert!(matches!(registry.check_access(&check), SynchronisedRegistryCheckAccessResult::ListsDenied));
    assert!(matches!(registry.unallow_whitelist(&RegistryUnallow {
        id: &owner, password: &owner_password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryWhitelistUnallowResult::Err));
    assert!(matches!(registry.unallow_whitelist(&RegistryUnallow {
        id: &owner, password: &Password::from(9), resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryWhitelistUnallowResult::VerificationFailure));
    assert!(matches!(registry.unallow_whitelist(&RegistryUnallow {
        id: &guest, password: &guest_password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryWhitelistUnallowResult::OwnershipDenied));
}
