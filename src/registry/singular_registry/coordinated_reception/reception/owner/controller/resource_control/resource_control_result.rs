pub enum ResourceControlVerificationResult {
    Verification(bool)
}

pub enum ResourceControlReleaseResult {
    Released(bool)
}

impl ResourceControlReleaseResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Released(true))
    }
}

pub enum ResourceControlOwnResult {
    Own(bool),
    OwnershipConflict
}