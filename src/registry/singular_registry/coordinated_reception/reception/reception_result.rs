use crate::prelude::{HostAccessPermissionResult, OwnerPasswordGeneratorResult};

pub enum ReceptionAccessPermissionResult {
    Host(HostAccessPermissionResult),
    Denied
}

pub enum ReceptionPasswordGeneratorResult<Password> {
    Owner(OwnerPasswordGeneratorResult<Password>)
}