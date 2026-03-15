use crate::prelude::{AccessControlBlacklistAllowResult, AccessControlBlacklistUnallowResult, AccessControlCheckAccessResult, AccessControlReleaseAllResult, AccessControlReleaseResult, AccessControlWhitelistAllowResult, AccessControlWhitelistUnallowResult, ResourceControlCheckOwnerResult, ResourceControlOwnResult, ResourceControlReleaseResult};

pub enum ControllerOwnResult {
    ResourceControl(ResourceControlOwnResult)
}

pub enum ControllerReleaseResourceResult {
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

pub enum ControllerUnallowBlacklistResult {
    Blacklist(AccessControlBlacklistUnallowResult),
    Denied
}

pub enum ControllerUnallowWhitelistResult {
    Whitelist(AccessControlWhitelistUnallowResult),
    Denied
}

pub enum ControllerCheckOwnerResult {
    ResourceControl(ResourceControlCheckOwnerResult)
}