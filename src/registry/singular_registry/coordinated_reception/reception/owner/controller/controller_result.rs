use crate::prelude::{AccessControlAccessResult, AccessControlBlacklistAllowResult, AccessControlReleaseResult, AccessControlWhitelistAllowResult, ResourceControlOwnResult, ResourceControlReleaseResult, ResourceControlVerificationResult};

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
    Verification(ResourceControlVerificationResult),
    AccessControl(AccessControlAccessResult)
}

impl ControllerAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            ControllerAccessResult::Verification(resource_control_verification_result) => resource_control_verification_result.ok(),
            ControllerAccessResult::AccessControl(access_control_access_result) => access_control_access_result.ok(),
        }
    }
}