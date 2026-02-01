use crate::prelude::RemoveAccessResult;

pub enum ReservationsAccessPermissionResult {
    Ok(bool)
}

impl ReservationsAccessPermissionResult {
    pub fn ok(&self) -> bool {
        match self {
            ReservationsAccessPermissionResult::Ok(ok) => *ok,
        }
    }
}

pub enum ReservationsReserveResult {
    FoundReserver,
    FirstReservation
}

pub enum ReservationsUnreserveResult {
    Accesses(RemoveAccessResult),
    NoReserver
}