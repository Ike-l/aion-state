use crate::{prelude::{AuthenticateRegistrationResult, AuthenticateUpdatePasswordResult, AuthenticationResult, ControllerBlacklistAllowResult, ControllerCheckAccessResult, ControllerOwnResult, ControllerReleaseIdResult, ControllerReleaseResourceResult, ControllerBlacklistUnallowResult, ControllerWhitelistUnallowResult, ControllerWhitelistAllowResult}, synchronised_registry::unsynchronised_registry::coordinated_reception::reception::owner::controller::controller_result::ControllerReleaseResourceAllResult};

pub enum OwnerOwnResult {
    Controller(ControllerOwnResult),
    Denied
}

pub enum OwnerRegisterResult {
    Authenticator(AuthenticateRegistrationResult)
}

pub enum OwnerReleaseResourceResult {
    Controller(ControllerReleaseResourceResult),
    Denied
}

pub enum OwnerUpdatePasswordResult {
    Authenticator(AuthenticateUpdatePasswordResult),
    Denied
}

pub enum OwnerUnregisterResult {
    Controller(ControllerReleaseIdResult),
    AuthenticatorUnregisterFailure,
    VerificationFailure,
}

pub enum OwnerWhitelistAllowResult {
    Controller(ControllerWhitelistAllowResult),
    Denied
}

pub enum OwnerBlacklistAllowResult<Password> {
    Controller(ControllerBlacklistAllowResult<Password>),
    Denied
}

pub enum OwnerCheckAccessResult {
    Controller(ControllerCheckAccessResult),
    Denied
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
    Denied
}

pub enum OwnerBlacklistUnallowResult {
    Controller(ControllerBlacklistUnallowResult),
    Denied
}

pub enum OwnerReleaseResourceAllResult{
    Controller(ControllerReleaseResourceAllResult),
    Denied
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