pub enum ManualRegistryAccessError {
    NotFound,
}

pub enum ManualRegistryReplacementResult<ReplacementResult> {
    Found(ReplacementResult),
    NotFound,
    DeniedAccess,
    NoOp
}

pub enum ManualRegistryReleaseResult {
    Storage(bool)
}

impl ManualRegistryReleaseResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Storage(true))
    }
}