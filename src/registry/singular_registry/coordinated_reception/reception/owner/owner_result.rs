use crate::prelude::{AuthenticateRegistrationResult, AuthenticateUpdatePasswordResult, ControllerOwnResult, ControllerReleaseResult};

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
    Unauthorised,
    Denied
}