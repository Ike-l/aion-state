use crate::prelude::{AccessControlBlacklistAllowResult, AccessControlBlacklistUnallowResult, AccessControlCheckAccessResult, AccessControlReleaseAllResult, AccessControlWhitelistAllowResult, AccessControlWhitelistUnallowResult, ResourceControlCheckOwnerResult, ResourceControlOwnResult, ResourceControlReleaseResult};

pub enum ControllerOwnResult {
    ResourceControl(ResourceControlOwnResult)
}

pub enum ControllerReleaseResourceResult {
    ResourceControl(ResourceControlReleaseResult),
    Denied
}

pub enum ControllerBlacklistAllowResult<Password> {
    Blacklist(AccessControlBlacklistAllowResult<Password>),
    Denied
}

pub enum ControllerWhitelistAllowResult {
    Whitelist(AccessControlWhitelistAllowResult),
    Denied
}

pub enum ControllerCheckAccessResult {
    AccessControl(AccessControlCheckAccessResult),
    IsOwner,
    NotOwned
}

impl ControllerCheckAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::IsOwner | Self::NotOwned => true,
            ControllerCheckAccessResult::AccessControl(access_control_access_result) => access_control_access_result.ok(),
        }
    }
}

pub enum ControllerReleaseIdResult {
    AccessControl(AccessControlReleaseAllResult)
}

pub enum ControllerBlacklistUnallowResult {
    Blacklist(AccessControlBlacklistUnallowResult),
    Denied
}

pub enum ControllerWhitelistUnallowResult {
    Whitelist(AccessControlWhitelistUnallowResult),
    Denied
}

pub enum ControllerCheckOwnerResult {
    ResourceControl(ResourceControlCheckOwnerResult)
}

pub enum ControllerReleaseResourceAllResult {
    All(Vec<ControllerReleaseResourceResult>)
}