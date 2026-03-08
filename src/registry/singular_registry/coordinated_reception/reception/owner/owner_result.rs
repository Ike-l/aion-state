use crate::prelude::{AuthenticateRegistrationResult, ControllerOwnResult, ControllerReleaseResult};

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