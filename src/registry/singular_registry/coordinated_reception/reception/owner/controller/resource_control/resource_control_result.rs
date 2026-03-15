pub enum ResourceControlCheckResourceOwnerResult {
    Verification(bool)
}

impl ResourceControlCheckResourceOwnerResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Verification(true))
    }
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

pub enum ResourceControlReleaseIdResult<T: Iterator> {
    Released(T)
}