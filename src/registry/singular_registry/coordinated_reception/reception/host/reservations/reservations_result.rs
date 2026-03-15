use crate::prelude::AccessesReleaseResult;

pub enum ReservationsCheckAccessResult {
    Ok(bool)
}

impl ReservationsCheckAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            ReservationsCheckAccessResult::Ok(ok) => *ok,
        }
    }
}

pub enum ReservationsReserveResult {
    FoundReserver,
    FirstReservation
}

pub enum ReservationsUnreserveResult {
    Accesses(AccessesReleaseResult),
    NoReserver
}