use crate::prelude::{AccessesDrainResult, AccessesReleaseResult};

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
    FirstReservation,
    Reservations(ReservationsCheckAccessResult)
}

#[derive(Debug)]
pub enum ReservationsUnreserveResult {
    Accesses(AccessesReleaseResult),
    NoReservationsMadeByReserver
}

impl ReservationsUnreserveResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Accesses(accesses) => accesses.ok(),
            Self::NoReservationsMadeByReserver => true
        }
    }
}

pub enum ReservationsDrainReservationsResult<T> {
    Accesses(AccessesDrainResult<T>),
    NoReserver
}