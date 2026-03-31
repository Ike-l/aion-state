use crate::prelude::{AccessControlBlacklistAllowResult, AccessControlBlacklistUnallowResult, AccessControlCheckAccessResult, AccessControlReleaseAllResult, AccessControlReleaseResult, AccessControlWhitelistAllowResult, AccessControlWhitelistUnallowResult, ResourceControlCheckOwnerResult, ResourceControlOwnResult, ResourceControlReleaseResult};

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
    Verification(ResourceControlCheckOwnerResult),
    AccessControl(AccessControlCheckAccessResult)
}

impl ControllerCheckAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            ControllerCheckAccessResult::Verification(resource_control_verification_result) => resource_control_verification_result.ok(),
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