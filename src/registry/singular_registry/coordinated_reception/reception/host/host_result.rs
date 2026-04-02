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

impl HostRecordAccessResult {
    pub fn ok(&self) -> bool {
        self.unreserve_result.as_ref().is_none_or(|host| host.ok()) &&
        self.record_access_result.ok()
    }
}

pub enum HostUnreserveResult {
    Reservations(ReservationsUnreserveResult)
}

impl HostUnreserveResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Reservations(reservations) => reservations.ok(),
        }
    }
}

pub enum HostReservationResult {
    Reservations(ReservationsReserveResult),
    AccessConflict(AccessesCheckAccessResult)
}

pub enum HostReleaseAccessResult {
    Accesses(AccessesReleaseResult)
}

pub enum HostDrainReservationsResult<T> {
    Reservations(ReservationsDrainReservationsResult<T>)
}