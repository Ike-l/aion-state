use crate::{prelude::{AuthenticateRegistrationResult, AuthenticateUnregisterResult, AuthenticateUpdatePasswordResult, AuthenticationResult, ControllerBlacklistAllowResult, ControllerCheckAccessResult, ControllerOwnResult, ControllerReleaseIdResult, ControllerReleaseResourceResult, ControllerBlacklistUnallowResult, ControllerWhitelistUnallowResult, ControllerWhitelistAllowResult}, registry::singular_registry::coordinated_reception::reception::owner::controller::controller_result::ControllerReleaseResourceAllResult};

pub enum OwnerOwnResult {
    Controller(ControllerOwnResult),
    Denied(AuthenticationResult),
}

pub enum OwnerRegisterResult {
    Authenticator(AuthenticateRegistrationResult)
}

pub enum OwnerReleaseResourceResult {
    Controller(ControllerReleaseResourceResult),
    Denied(AuthenticationResult)
}

pub enum OwnerUpdatePasswordResult {
    Authenticator(AuthenticateUpdatePasswordResult),
    Denied(AuthenticationResult)
}

pub enum OwnerUnregisterResult {
    Controller(ControllerReleaseIdResult),
    Authenticator(AuthenticateUnregisterResult),
    Denied(AuthenticationResult)
}

pub enum OwnerWhitelistAllowResult {
    Controller(ControllerWhitelistAllowResult),
    Denied(AuthenticationResult)
}

pub enum OwnerBlacklistAllowResult<Password> {
    Controller(ControllerBlacklistAllowResult<Password>),
    Denied(AuthenticationResult)
}

pub enum OwnerCheckAccessResult {
    Controller(ControllerCheckAccessResult),
    Denied(AuthenticationResult),
    NeedIdPassword,
}

impl OwnerCheckAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            OwnerCheckAccessResult::Controller(controller_check_access_result) => controller_check_access_result.ok(),
            _ => false
        }
    }
}

pub enum OwnerWhitelistUnallowResult {
    Controller(ControllerWhitelistUnallowResult),
    Denied(AuthenticationResult)
}

pub enum OwnerBlacklistUnallowResult {
    Controller(ControllerBlacklistUnallowResult),
    Denied(AuthenticationResult)
}

pub enum OwnerReleaseResourceAllResult{
    Controller(ControllerReleaseResourceAllResult),
    Denied(AuthenticationResult)
}

pub enum OwnerAuthenticationResult {
    Authenticator(AuthenticationResult)
}

impl OwnerAuthenticationResult {
    pub fn ok(&self) -> bool {
        match self {
            OwnerAuthenticationResult::Authenticator(authentication_result) => authentication_result.ok(),
        }
    }
}