use aion_state::prelude::{RegistryDrainReservations, RegistryReservation, SynchronisedRegistryDrainReservationsResult, SynchronisedRegistryReservationResult};
use crate::default::prelude::*;
use super::owned_registry;

#[test]
fn drain_returns_and_clears_reservations() {
    let (registry, id, password, resource_id) = owned_registry();

    assert!(matches!(registry.drain_reservations(&RegistryDrainReservations { id: &id, password: &password }), SynchronisedRegistryDrainReservationsResult::NoReserver));
    assert!(matches!(registry.reserve(RegistryReservation {
        id: id.clone(), id_password: &password, resource_id: resource_id.clone(),
        access: Access::Shared(2), password: None,
    }), SynchronisedRegistryReservationResult::OkNew));
    assert!(matches!(registry.drain_reservations(&RegistryDrainReservations { id: &id, password: &Password::from(2) }), SynchronisedRegistryDrainReservationsResult::VerificationFailure));
    let result = registry.drain_reservations(&RegistryDrainReservations { id: &id, password: &password });
    let SynchronisedRegistryDrainReservationsResult::Drain(reservations) = result else {
        panic!("expected stored reservations to be drained");
    };
    assert_eq!(reservations, vec![(resource_id.clone(), Access::Shared(2))]);
    assert!(matches!(registry.drain_reservations(&RegistryDrainReservations { id: &id, password: &password }), SynchronisedRegistryDrainReservationsResult::Drain(reservations) if reservations.is_empty()));
}
