use crate::prelude::HostAccessPermissionResult;

pub enum ReceptionAccessPermissionResult {
    Host(HostAccessPermissionResult),
    Denied
}