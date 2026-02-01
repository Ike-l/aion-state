pub struct OwnerAccessPermissionInput<'a, OwnerId, OwnerKey> {
    pub owner_credentials: Option<(&'a OwnerId, &'a OwnerKey)>,
}