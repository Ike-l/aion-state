pub struct OwnerAccessPermissionInput<'a, OwnerId, OwnerKey, Password, Access> {
    pub owner_credentials: Option<(&'a OwnerId, &'a OwnerKey)>,
    pub password: Option<&'a Password>,
    pub access: &'a Access
}

pub struct OwnerPasswordGeneratorInput<'a, OwnerId, OwnerKey, Access, Policy> {
    pub owner_id: &'a OwnerId,
    pub owner_key: &'a OwnerKey,
    pub access: &'a Access,
    pub policy: &'a Policy
}