pub enum AuthenticationResult {
    Verification(bool)
}

impl AuthenticationResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Verification(true))
    }
}

pub enum AuthenticateRegistrationResult {
    Registration(bool)
}

pub enum AuthenticateUpdatePasswordResult {
    Updated(bool),
    Denied
}

pub enum AuthenticateUnregisterResult {
    Unregister(bool)
}

impl AuthenticateUnregisterResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Unregister(true))
    }
}