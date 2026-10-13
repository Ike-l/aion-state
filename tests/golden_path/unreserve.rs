use aion_state::prelude::{RegistryReservation, RegistryUnreserve, SynchronisedRegistryReservationResult, SynchronisedRegistryUnreserveResult};
use crate::default::prelude::*;
use super::owned_registry;

#[test]
fn unreserve_releases_only_the_requested_access_and_reports_failures() {
    let (registry, id, password, resource_id) = owned_registry();
    let missing_id = ResourceId::new_label("unreserved-resource");

    assert!(matches!(registry.unreserve(&RegistryUnreserve {
        id: &id, password: &password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryUnreserveResult::NoReservationsMadeByReserver));
    assert!(matches!(registry.reserve(RegistryReservation {
        id: id.clone(), id_password: &password, resource_id: resource_id.clone(),
        access: Access::Shared(2), password: None,
    }), SynchronisedRegistryReservationResult::OkNew));
    assert!(matches!(registry.unreserve(&RegistryUnreserve {
        id: &id, password: &password, resource_id: &missing_id, access: &Access::Shared(1),
    }), SynchronisedRegistryUnreserveResult::NoReservation));
    assert!(matches!(registry.unreserve(&RegistryUnreserve {
        id: &id, password: &password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryUnreserveResult::Ok));
    assert!(matches!(registry.unreserve(&RegistryUnreserve {
        id: &id, password: &Password::from(2), resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryUnreserveResult::VerificationFailure));
    assert!(matches!(registry.unreserve(&RegistryUnreserve {
        id: &id, password: &password, resource_id: &resource_id, access: &Access::Shared(1),
    }), SynchronisedRegistryUnreserveResult::Ok));
}
