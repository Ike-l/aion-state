use std::fmt::Debug;

use tracing::{Level, event, span};

use crate::prelude::{AccessStorage, Accesses, Accessor, FUNCTION_LEVEL, PermitsAccessInput, RecordAccessInput, RemoveAccessInput, ReservationStorage, ReservationsAccessPermissionInput, ReservationsAccessPermissionResult, ReservationsReserveResult, ReservationsUnreserveResult, ReserveInput, UnreserveInput};

pub mod reservations_input;
pub mod reservations_result;
pub mod reservation_storage;

pub struct Reservations<RS> {
    reservation_storage: RS,
}

impl<
    RS: ReservationStorage<AccessStorage = AS>, 
    AS: AccessStorage + Default
> Reservations<RS> 
    where 
        RS::Key: Debug + PartialEq,
        AS::Value: Debug + Accessor
{
    pub fn permits_access(
        &self,
        ReservationsAccessPermissionInput {
            reserver, access_key, access,
        }: ReservationsAccessPermissionInput<'_, RS::Key, AS::Key, AS::Value>
    ) -> ReservationsAccessPermissionResult {
        let span = span!(FUNCTION_LEVEL, "Reservations Permits Access");
        let _enter = span.enter();

        let conflicts = self.reservation_storage
            .iter()
            .any(|(current_reserver, access_map)| {
                let is_current_reserver = reserver
                    .is_some_and(
                        |reserver| reserver == current_reserver
                    );

                if !is_current_reserver {
                    let permission = access_map.permits_access(PermitsAccessInput { access_key, access });
                    if permission.ok() {
                        false
                    } else {
                        // warn here because 
                        // 1. It early returns- could be more unknown conflicts
                        // 2. Because of 1. the return doesn't include the conflict info so can put it here
                        event!(Level::WARN, conflicting_reserver =? reserver, "Reservation Conflict");
                        true
                    }
                } else {
                    false
                }
            });

        ReservationsAccessPermissionResult::Ok(!conflicts)
    }

    pub fn reserve(
        &mut self,
        ReserveInput {
            reserver, access_key, access
        }: ReserveInput<RS::Key, AS::Key, AS::Value>
    ) -> ReservationsReserveResult {
        let span = span!(FUNCTION_LEVEL, "Reservations Reserve");
        let _enter = span.enter();

        let input = RecordAccessInput { access, access_key };
        if let Some(access_map) = self.reservation_storage.get_mut(&reserver) {
            access_map.record_access(input);
            ReservationsReserveResult::FoundReserver
        } else {
            let mut access_map = Accesses::default();
            access_map.record_access(input);
            self.reservation_storage.insert(reserver, access_map);
            ReservationsReserveResult::FirstReservation
        }
    }

    pub fn unreserve(
        &mut self,
        UnreserveInput {
            reserver, access_key, access
        }: UnreserveInput<'_, RS::Key, AS::Key, AS::Value>
    ) -> ReservationsUnreserveResult {
        let span = span!(FUNCTION_LEVEL, "Reservations Unreserve");
        let _enter = span.enter();

        if let Some(access_map) = self.reservation_storage.get_mut(reserver) {
            ReservationsUnreserveResult::Accesses(access_map.remove_access(RemoveAccessInput { access, access_key }))
        } else {
            ReservationsUnreserveResult::NoReserver
        }
    }
}