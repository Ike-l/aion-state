pub enum ManualRegistryAccessError {
    NotFound,
}

pub enum ManualRegistryReplacementResult<ReplacementResult> {
    Found(ReplacementResult),
    NotFound,
    DeniedAccess,
    NoOp,   
}

pub enum ManualRegistryCheckedReplacementResult<ReplacementResult> {
    ReplacementResult(ManualRegistryReplacementResult<ReplacementResult>),
    RemovalReallocates,
    InsertingReallocates,
}

pub enum ManualRegistryReleaseResult {
    Storage(bool)
}
