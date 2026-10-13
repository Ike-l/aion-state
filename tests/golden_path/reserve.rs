use aion_state::prelude::{RegistryAllow, RegistryRegister, RegistryReservation, SynchronisedRegistryBlacklistAllowResult, SynchronisedRegistryReservationResult};
use crate::default::prelude::*;
use super::owned_registry;

#[cfg(feature = "notifier")]
use aion_state::prelude::{RegistryAcquireAccess, RegistryReleaseAccess, SynchronisedRegistryReleaseAccessResult};
#[cfg(feature = "notifier")]
use crate::default::storages::registry_storage::ResourceWrapper;

#[test]
fn reserve_records_access_and_reports_failures() {
    let (registry, id, password, resource_id) = owned_registry();
    let access = Access::Shared(1);
    let guest = ReserverId::new("golden-reserver");
    let guest_password = Password::from(3);
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: guest_password.clone() }).ok());
    let blacklist_password = match registry.allow_blacklist(RegistryAllow {
        id: &id, password: &password, resource_id: resource_id.clone(), access: Access::Unique,
    }) {
        SynchronisedRegistryBlacklistAllowResult::Ok(password) => password,
        result => panic!("expected blacklist entry, got {result:?}"),
    };
    let wrong_resource_password = [0u64, 1].into_iter().map(Password::new)
        .find(|candidate| candidate != &blacklist_password).unwrap();
    assert!(matches!(registry.reserve(RegistryReservation {
        id: guest.clone(), id_password: &guest_password, resource_id: resource_id.clone(),
        access: Access::Unique, password: Some(&wrong_resource_password),
    }), SynchronisedRegistryReservationResult::ListsDenied));

    #[cfg(feature = "notifier")]
    {
        let held = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
            user_details: Some((&id, &password)), resource_id: resource_id.clone(),
            access: Access::Unique, password: None,
        }).unwrap();
        assert!(matches!(held, AccessResult::Unique(_)));
        drop(held);
        assert!(matches!(registry.reserve(RegistryReservation {
            id: id.clone(), id_password: &password, resource_id: resource_id.clone(),
            access: access.clone(), password: None,
        }), SynchronisedRegistryReservationResult::AccessConflict));
        unsafe {
            assert!(matches!(registry.release_access(&RegistryReleaseAccess {
                resource_id: &resource_id, access: &Access::Unique,
            }), SynchronisedRegistryReleaseAccessResult::Ok));
        }
    }

    assert!(matches!(registry.reserve(RegistryReservation {
        id: id.clone(), id_password: &password, resource_id: resource_id.clone(),
        access: access.clone(), password: None,
    }), SynchronisedRegistryReservationResult::OkNew));
    assert!(matches!(registry.reserve(RegistryReservation {
        id: id.clone(), id_password: &password, resource_id: resource_id.clone(),
        access: access.clone(), password: None,
    }), SynchronisedRegistryReservationResult::Ok));
    assert!(matches!(registry.reserve(RegistryReservation {
        id: id.clone(),
        id_password: &Password::from(2),
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }), SynchronisedRegistryReservationResult::VerificationFailure));
    assert!(registry.allow_whitelist(RegistryAllow {
        id: &id, password: &password, resource_id: resource_id.clone(), access: Access::Unique,
    }).ok());
    assert!(matches!(registry.reserve(RegistryReservation {
        id: guest, id_password: &guest_password, resource_id, access: Access::Unique, password: None,
    }), SynchronisedRegistryReservationResult::ReservationConflict));
}
