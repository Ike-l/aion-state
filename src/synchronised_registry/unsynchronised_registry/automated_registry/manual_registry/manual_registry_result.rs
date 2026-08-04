pub enum ManualRegistryAccessError {
    NotFound,
}

pub enum ManualRegistryReplacementResult<ReplacementResult> {
    Found(ReplacementResult),
    NotFound,
    DeniedAccess,
    NoOp,
    RemovalReallocates,
    InsertingReallocates,
}

pub enum ManualRegistryReleaseResult {
    Storage(bool)
}
