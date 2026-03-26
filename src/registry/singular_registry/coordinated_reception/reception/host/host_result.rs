use crate::prelude::{AccessesCheckAccessResult, AccessesRecordAccessResult, AccessesReleaseResult, ReservationsDrainReservationsResult, ReservationsReserveResult, ReservationsUnreserveResult};

pub enum HostCheckAccessResult {
    Accesses(AccessesCheckAccessResult),
    ReservationConflict,
}

impl HostCheckAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Accesses(accesses) => accesses.ok(),
            _ => false
        }
    }
}

pub struct HostRecordAccessResult {
    pub unreserve_result: Option<HostUnreserveResult>,
    pub record_access_result: AccessesRecordAccessResult
}

pub enum HostUnreserveResult {
    Reservations(ReservationsUnreserveResult)
}

pub enum HostReservationResult {
    Reservations(ReservationsReserveResult)
}

pub enum HostReleaseAccessResult {
    Accesses(AccessesReleaseResult)
}

pub enum HostDrainReservationsResult<T> {
    Reservations(ReservationsDrainReservationsResult<T>)
}