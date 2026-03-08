use crate::prelude::{AccessControlAccessResult, AccessControlBlacklistAllowResult, AccessControlReleaseResult, AccessControlWhitelistAllowResult, ResourceControlOwnResult, ResourceControlReleaseResult};

pub enum ControllerOwnResult {
    ResourceControl(ResourceControlOwnResult)
}

pub enum ControllerReleaseResult {
    ResourceControl(ResourceControlReleaseResult),
    AccessControl(AccessControlReleaseResult)
}

pub enum ControllerBlacklistAllowResult<Password> {
    Blacklist(AccessControlBlacklistAllowResult<Password>),
    Denied
}

pub enum ControllerWhitelistAllowResult {
    Whitelist(AccessControlWhitelistAllowResult),
    Denied
}

pub enum ControllerAccessResult {
    AccessControl(AccessControlAccessResult)
}