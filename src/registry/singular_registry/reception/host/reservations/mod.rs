use std::marker::PhantomData;

use tracing::{Level, event, span};

use crate::prelude::{FUNCTION_LEVEL, ReservationMap, ReservationMapAccessPermissionInput, ReservationMapAccessPermissionResult, Storage};

pub mod reservation_map;
pub mod reservations_input;
pub mod reservations_result;

pub struct Reservations<S, R> {
    reservation_map: parking_lot::RwLock<R>,
    _s: PhantomData<S>
}

impl<S: Storage, R: ReservationMap<S>> Reservations<S, R> {
    pub fn permits_access(
        &self,
        ReservationMapAccessPermissionInput {
            reserver, access_key, access,
        }: ReservationMapAccessPermissionInput<'_, R::Reserver, S::Key, S::Value>
    ) -> ReservationMapAccessPermissionResult {
        let span = span!(FUNCTION_LEVEL, "Reservations Permits Access");
        let _enter = span.enter();

        let conflicts = self.reservation_map
            .read()
            .iter()
            .any(|(current_reserver, access_map)| {
                let is_current_reserver = reserver
                    .is_some_and(
                        |reserver| reserver == current_reserver
                    );

                if !is_current_reserver {
                    let permission = access_map.permits_access(access_key, access);
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
}