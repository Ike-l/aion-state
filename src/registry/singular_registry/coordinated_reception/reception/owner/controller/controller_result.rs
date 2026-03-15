use crate::prelude::{AccessControlCheckAccessResult, AccessControlBlacklistAllowResult, AccessControlReleaseAllResult, AccessControlReleaseResult, AccessControlWhitelistAllowResult, ResourceControlOwnResult, ResourceControlReleaseResult, ResourceControlCheckResourceOwnerResult};

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

pub enum ControllerCheckAccessResult {
    Verification(ResourceControlCheckResourceOwnerResult),
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