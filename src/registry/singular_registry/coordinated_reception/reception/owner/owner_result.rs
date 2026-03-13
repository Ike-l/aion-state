use crate::prelude::{AuthenticateRegistrationResult, AuthenticateUnregisterResult, AuthenticateUpdatePasswordResult, ControllerAccessResult, ControllerBlacklistAllowResult, ControllerOwnResult, ControllerReleaseIdResult, ControllerReleaseResult, ControllerWhitelistAllowResult};

pub enum OwnerOwnResult {
    Controller(ControllerOwnResult),
    Denied,
}

pub enum OwnerRegisterResult {
    Authenticator(AuthenticateRegistrationResult)
}

pub enum OwnerReleaseResult {
    Controller(ControllerReleaseResult),
    Denied
}

pub enum OwnerUpdatePasswordResult {
    Authenticator(AuthenticateUpdatePasswordResult),
    Denied
}

pub enum OwnerUnregisterResult {
    Controller(ControllerReleaseIdResult),
    Authenticator(AuthenticateUnregisterResult),
    Denied
}

pub enum OwnerWhitelistAllowResult {
    Controller(ControllerWhitelistAllowResult),
    Denied
}

pub enum OwnerBlacklistAllowResult<Password> {
    Controller(ControllerBlacklistAllowResult<Password>),
    Denied
}

pub enum OwnerAccessResult {
    Controller(ControllerAccessResult)
}