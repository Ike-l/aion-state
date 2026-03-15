use crate::prelude::{AuthenticateRegistrationResult, AuthenticateUnregisterResult, AuthenticateUpdatePasswordResult, ControllerCheckAccessResult, ControllerBlacklistAllowResult, ControllerOwnResult, ControllerReleaseIdResult, ControllerReleaseResourceResult, ControllerWhitelistAllowResult};

pub enum OwnerOwnResult {
    Controller(ControllerOwnResult),
    Denied,
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

pub enum OwnerCheckAccessResult {
    Controller(ControllerCheckAccessResult)
}