use crate::prelude::{AccessPermission, RecordAccessResult, RemoveAccessResult, ReservationsReserveResult, ReservationsUnreserveResult};

pub enum HostAccessPermissionResult {
    Accesses(AccessPermission),
    ReservationConflict,
}

pub struct HostRecordAccessResult {
    pub unreserve_result: Option<HostUnreserveResult>,
    pub record_access_result: RecordAccessResult
}

pub enum HostUnreserveResult {
    Reservations(ReservationsUnreserveResult)
}

pub enum HostReservationResult {
    Reservations(ReservationsReserveResult)
}

pub enum HostReleaseAccessResult {
    Accesses(RemoveAccessResult)
}