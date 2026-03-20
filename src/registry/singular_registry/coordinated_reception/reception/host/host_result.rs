use crate::prelude::{AccessesCheckAccessResult, AccessesRecordAccessResult, AccessesReleaseResult, ReservationsDrainReservationsResult, ReservationsReserveResult, ReservationsUnreserveResult};

pub enum HostCheckAccessResult {
    Accesses(AccessesCheckAccessResult),
    ReservationConflict,
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