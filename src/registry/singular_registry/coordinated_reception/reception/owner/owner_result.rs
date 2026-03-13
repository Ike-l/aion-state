use crate::prelude::{AuthenticateRegistrationResult, AuthenticateUnregisterResult, AuthenticateUpdatePasswordResult, ControllerOwnResult, ControllerReleaseIdResult, ControllerReleaseResult};

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