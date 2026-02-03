use crate::prelude::{AccessPermission, RecordAccessResult, ReservationsUnreserveResult};

pub enum HostAccessPermissionResult {
    Accesses(AccessPermission),
    ReservationConflict,
}

pub struct HostRecordAccessResult {
    pub unreserve_result: Option<ReservationsUnreserveResult>,
    pub record_access_result: RecordAccessResult
}