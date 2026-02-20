use std::fmt::Debug;

use tracing::{Level, event};

use crate::prelude::{AccessStorage, Accesses, Accessor, PermitsAccessInput, RecordAccessInput, RemoveAccessInput, ReservationStorage, ReservationsAccessPermissionInput, ReservationsAccessPermissionResult, ReservationsReserveResult, ReservationsUnreserveResult, ReserveInput, UnreserveInput, trace_function};

pub mod reservations_input;
pub mod reservations_result;
pub mod reservation_storage;

// in future want another layer to track reservation users? - i.e to unreserve you must be the same person who reserved it

/// Wraps reservation storage with `Accessor` semantics
pub struct Reservations<RS> {
    reservation_storage: RS,
}

impl<
    RS: ReservationStorage<AccessStorage = AS>, 
    AS: AccessStorage + Default
> Reservations<RS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor
{
    /// A `Conflict` occurs if all:
    /// 
    /// There is an existing reserver
    /// The existing reserver is not the incoming reserver
    /// The existing reserver's access denies the incoming access using `Accessor`
    /// 
    /// Returns on first conflict found
    pub fn permits_access(
        &self,
        ReservationsAccessPermissionInput {
            reserver_id, access_id, access,
        }: ReservationsAccessPermissionInput<'_, RS::ReserverId, AS::ValueId, AS::Access>
    ) -> ReservationsAccessPermissionResult {
        trace_function!("Reservations Permits Access");

        let conflicts = self.reservation_storage
            .iter()
            .any(|(current_reserver, access_map)| {
                let is_current_reserver = reserver_id
                    .is_some_and(
                        |reserver| reserver == current_reserver
                    );

                if !is_current_reserver {
                    let permission = access_map.permits_access(PermitsAccessInput { access_id, access });
                    if permission.ok() {
                        false
                    } else {
                        // warn here because 
                        // 1. It early returns- could be more unknown conflicts
                        // 2. Because of 1. the return doesn't include the conflict info so can put it here
                        event!(Level::WARN, conflicting_reserver =? current_reserver, "Reservation Conflict");
                        true
                    }
                } else {
                    false
                }
            });

        ReservationsAccessPermissionResult::Ok(!conflicts)
    }

    /// Reserves by recording the access with the associated reserver
    /// 
    /// If the first reservation by the reserver creates an Access container using the `Default` trait
    pub fn reserve(
        &mut self,
        ReserveInput {
            reserver_id, access_id, access
        }: ReserveInput<RS::ReserverId, AS::ValueId, AS::Access>
    ) -> ReservationsReserveResult {
        trace_function!("Reservations Reserve");

        let input = RecordAccessInput { access, access_id };
        if let Some(access_map) = self.reservation_storage.get_mut(&reserver_id) {
            access_map.record_access(input);
            ReservationsReserveResult::FoundReserver
        } else {
            let mut access_map = Accesses::default();
            access_map.record_access(input);
            self.reservation_storage.insert(reserver_id, access_map);
            ReservationsReserveResult::FirstReservation
        }
    }

    /// Unreserves by releasing the access corresponding with the incoming reserver
    pub fn unreserve(
        &mut self,
        UnreserveInput {
            reserver_id, access_id, access
        }: UnreserveInput<'_, RS::ReserverId, AS::ValueId, AS::Access>
    ) -> ReservationsUnreserveResult {
        trace_function!("Reservations Unreserve");

        if let Some(access_map) = self.reservation_storage.get_mut(reserver_id) {
            ReservationsUnreserveResult::Accesses(access_map.release_access(RemoveAccessInput { access, access_id }))
        } else {
            ReservationsUnreserveResult::NoReserver
        }
    }
}