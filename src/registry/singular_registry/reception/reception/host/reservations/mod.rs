use std::fmt::Debug;

use tracing::{Level, event, span};

use crate::prelude::{AccessStorage, Accesses, FUNCTION_LEVEL, PermitsAccessInput, RecordAccessInput, ReservationMapAccessPermissionResult, ReservationStorage, ReservationsAccessPermissionInput, ReserveInput};

pub mod reservations_input;
pub mod reservations_result;
pub mod reservation_storage;

pub struct Reservations<RS> {
    reservation_map: RS,
}

impl<
    RS: ReservationStorage<AccessStorage = AS>, 
    AS: AccessStorage + Default
> Reservations<RS> 
where RS::Key: PartialEq + Debug
{
    pub fn permits_access(
        &self,
        ReservationsAccessPermissionInput {
            reserver, access_key, access,
        }: ReservationsAccessPermissionInput<'_, RS::Key, AS::Key, AS::Value>
    ) -> ReservationMapAccessPermissionResult {
        let span = span!(FUNCTION_LEVEL, "Reservations Permits Access");
        let _enter = span.enter();

        let conflicts = self.reservation_map
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
                        event!(Level::WARN, conflicting_reserver =? reserver, "Reservation Conflict");
                        true
                    }
                } else {
                    false
                }
            });

        ReservationMapAccessPermissionResult::Ok(!conflicts)
    }

    pub fn reserve(
        &mut self,
        ReserveInput {
            reserver, access_key, access
        }: ReserveInput<RS::Key, AS::Key, AS::Value>
    ) {
        let input = RecordAccessInput { access, access_key };
        if let Some(access_map) = self.reservation_map.get_mut(&reserver) {
            access_map.record_access(input);
        } else {
            let mut access_map = Accesses::default();
            access_map.record_access(input);
            self.reservation_map.insert(reserver, access_map);
        }
    }

    pub fn unreserve(
        &self
    ) {

    }
}