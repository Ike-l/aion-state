pub enum AuthenticationResult {
    OwnershipVerification(bool),
    Denied,
}

impl AuthenticationResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::OwnershipVerification(true))
    }
}