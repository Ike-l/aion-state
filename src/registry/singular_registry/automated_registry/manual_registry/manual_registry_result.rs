pub enum ManualRegistryAccessResult<AccessResult> {
    Found(AccessResult),
    NotFound,
}

pub enum ManualRegistryReplacementResult<ReplacementResult> {
    Found(ReplacementResult),
    NotFound,
    DeniedAccess,
    NoOp
}

pub enum ManualRegistryCheckAccessResult {
    NotFound,
    Found
}

impl ManualRegistryCheckAccessResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Found)
    }
}