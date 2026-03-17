use crate::prelude::{AuthenticateRegistrationResult, AuthenticateUnregisterResult, AuthenticateUpdatePasswordResult, AuthenticationResult, ControllerBlacklistAllowResult, ControllerCheckAccessResult, ControllerOwnResult, ControllerReleaseIdResult, ControllerReleaseResourceResult, ControllerUnallowBlacklistResult, ControllerUnallowWhitelistResult, ControllerWhitelistAllowResult};

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
    Controller(ControllerCheckAccessResult)
}

pub enum OwnerUnallowWhitelistResult {
    Controller(ControllerUnallowWhitelistResult),
    Denied(AuthenticationResult)
}

pub enum OwnerUnallowBlacklistResult {
    Controller(ControllerUnallowBlacklistResult),
    Denied(AuthenticationResult)
}