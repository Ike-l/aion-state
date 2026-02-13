pub enum AuthenticationResult {
    Ok,
    Denied,
    OwnershipError,
    OwnerError
}

impl AuthenticationResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Ok)
    }
}