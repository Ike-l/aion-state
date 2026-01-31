pub enum SingularRegistryAccessResult<AccessResult, Permission> {
    OkAccess(AccessResult),
    DeniedAccess(Permission),
}
