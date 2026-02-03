pub struct OwnerAccessPermissionInput<'a, OwnerId, OwnerKey, Item, Password, Access> {
    pub owner_credentials: Option<(&'a OwnerId, &'a OwnerKey)>,
    pub item: &'a Item,
    pub password: Option<&'a Password>,
    pub access: &'a Access
}

pub struct OwnerPasswordGeneratorInput<'a, OwnerId, OwnerKey, Item, Access, Policy> {
    pub owner_id: &'a OwnerId,
    pub owner_key: &'a OwnerKey,
    pub item: &'a Item,
    pub access: &'a Access,
    pub policy: &'a Policy
}