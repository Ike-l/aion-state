pub struct OwnerAccessPermissionInput<'a, OwnerId, OwnerKey, Password, Access> {
    pub owner_credentials: Option<(&'a OwnerId, &'a OwnerKey)>,
    pub password: Option<&'a Password>,
    pub access: &'a Access
}