use crate::prelude::{HostAccessPermissionResult, HostReservationResult, HostUnreserveResult, OwnerPasswordGeneratorResult};

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

pub enum ReceptionUnreserveResult {
    Host(HostUnreserveResult)
}