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
