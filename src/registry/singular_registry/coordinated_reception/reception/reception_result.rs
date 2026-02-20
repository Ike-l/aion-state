use crate::prelude::{HostAccessPermissionResult, HostReservationResult, OwnerPasswordGeneratorResult};

pub enum ReceptionAccessPermissionResult {
    Host(HostAccessPermissionResult),
    Denied
}

pub enum ReceptionPasswordGeneratorResult<Password> {
    Owner(OwnerPasswordGeneratorResult<Password>)
}

pub enum ReceptionReservationResult {
    Host(HostReservationResult),
    Denied,
}