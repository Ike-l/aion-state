use crate::prelude::AccessPermission;

pub enum HostAccessPermissionResult {
    Accesses(AccessPermission),
    ReservationConflict,
}