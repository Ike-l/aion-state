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