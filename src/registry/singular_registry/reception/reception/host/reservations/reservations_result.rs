use crate::prelude::RemoveAccessResult;

pub enum ReservationsAccessPermissionResult {
    Ok(bool)
}

pub enum ReservationsReserveResult {
    FoundReserver,
    FirstReservation
}

pub enum ReservationsUnreserveResult {
    Accesses(RemoveAccessResult),
    NoReserver
}