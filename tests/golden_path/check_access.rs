use aion_state::prelude::{RegistryAllow, RegistryCheckAccess, RegistryRegister, RegistryReservation, SynchronisedRegistryCheckAccessResult, SynchronisedRegistryReservationResult};
use crate::default::prelude::*;
use super::owned_registry;

#[cfg(feature = "notifier")]
use aion_state::prelude::{RegistryAcquireAccess, RegistryDrainReservations, RegistryReleaseAccess, SynchronisedRegistryDrainReservationsResult};
#[cfg(feature = "notifier")]
use crate::default::storages::registry_storage::ResourceWrapper;

#[test]
fn check_access_reports_each_access_denial_and_resource_state() {
    let (registry, id, password, resource_id) = owned_registry();
    let missing_id = ResourceId::new_label("missing-resource");
    let owner_shared = RegistryCheckAccess {
        user_details: Some((&id, &password)), resource_id: &resource_id,
        access: &Access::Shared(1), password: None,
    };
    assert!(matches!(registry.check_access(&owner_shared), SynchronisedRegistryCheckAccessResult::ContainsResource));
    assert!(matches!(registry.check_access(&RegistryCheckAccess {
        user_details: None, resource_id: &missing_id, access: &Access::Shared(1), password: None,
    }), SynchronisedRegistryCheckAccessResult::MissingResource));
    assert!(matches!(registry.check_access(&RegistryCheckAccess {
        user_details: Some((&id, &Password::from(2))), resource_id: &resource_id,
        access: &Access::Shared(1), password: None,
    }), SynchronisedRegistryCheckAccessResult::VerificationFailure));

    let guest = ReserverId::new("check-access-guest");
    let guest_password = Password::from(3);
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: guest_password.clone() }).ok());
    assert!(matches!(registry.check_access(&RegistryCheckAccess {
        user_details: Some((&guest, &guest_password)), resource_id: &resource_id,
        access: &Access::Shared(1), password: None,
    }), SynchronisedRegistryCheckAccessResult::ListsDenied));

    assert!(matches!(registry.reserve(RegistryReservation {
        id: id.clone(), id_password: &password, resource_id: resource_id.clone(),
        access: Access::Shared(1), password: None,
    }), SynchronisedRegistryReservationResult::OkNew));
    assert!(registry.allow_whitelist(RegistryAllow {
        id: &id, password: &password, resource_id: resource_id.clone(), access: Access::Unique,
    }).ok());
    assert!(matches!(registry.check_access(&RegistryCheckAccess {
        user_details: Some((&guest, &guest_password)), resource_id: &resource_id,
        access: &Access::Unique, password: None,
    }), SynchronisedRegistryCheckAccessResult::ReservationConflict));

    #[cfg(feature = "notifier")]
    {
        assert!(matches!(registry.drain_reservations(&RegistryDrainReservations { id: &id, password: &password }),
            SynchronisedRegistryDrainReservationsResult::Drain(reservations)
                if reservations == vec![(resource_id.clone(), Access::Shared(1))]));
        let active = registry.acquire_access::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
            user_details: Some((&id, &password)), resource_id: resource_id.clone(),
            access: Access::Shared(1), password: None,
        }).unwrap();
        assert!(matches!(active, AccessResult::Shared(_)));
        drop(active);
        assert!(matches!(registry.check_access(&RegistryCheckAccess {
            user_details: Some((&guest, &guest_password)), resource_id: &resource_id,
            access: &Access::Unique, password: None,
        }), SynchronisedRegistryCheckAccessResult::Err));
        assert!(matches!(unsafe { registry.release_access(&RegistryReleaseAccess {
            resource_id: &resource_id, access: &Access::Shared(1),
        }) }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
    }
}
