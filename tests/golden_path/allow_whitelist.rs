use aion_state::prelude::{RegistryAllow, RegistryCheckAccess, RegistryRegister, RegistryUnallow, SynchronisedRegistryCheckAccessResult, SynchronisedRegistryWhitelistAllowResult, SynchronisedRegistryWhitelistUnallowResult};
use crate::default::prelude::*;
use super::owned_registry;

#[test]
fn whitelist_allow_changes_non_owner_access_and_can_be_removed() {
    let (registry, owner, owner_password, resource_id) = owned_registry();
    let guest = ReserverId::new("golden-guest");
    let guest_password = Password::from(2);
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: guest_password }).ok());
    assert!(matches!(registry.allow_whitelist(RegistryAllow {
        id: &guest, password: &Password::from(2), resource_id: resource_id.clone(), access: Access::Shared(1),
    }), SynchronisedRegistryWhitelistAllowResult::OwnershipDenied));
    assert!(matches!(registry.allow_whitelist(RegistryAllow {
        id: &owner, password: &Password::from(9), resource_id: resource_id.clone(), access: Access::Shared(1),
    }), SynchronisedRegistryWhitelistAllowResult::VerificationFailure));
    let allow = RegistryAllow {
        id: &owner,
        password: &owner_password,
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
    };
    assert!(matches!(registry.allow_whitelist(allow), SynchronisedRegistryWhitelistAllowResult::Ok));
    let check = RegistryCheckAccess {
        user_details: Some((&guest, &Password::from(2))),
        resource_id: &resource_id,
        access: &Access::Shared(1),
        password: None,
    };
    assert!(matches!(registry.check_access(&check), SynchronisedRegistryCheckAccessResult::ContainsResource));
    assert!(matches!(registry.unallow_whitelist(&RegistryUnallow {
        id: &owner, password: &owner_password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryWhitelistUnallowResult::Ok));
    assert!(matches!(registry.check_access(&check), SynchronisedRegistryCheckAccessResult::ListsDenied));
}
