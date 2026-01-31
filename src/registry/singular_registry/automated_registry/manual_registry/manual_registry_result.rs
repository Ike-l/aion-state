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