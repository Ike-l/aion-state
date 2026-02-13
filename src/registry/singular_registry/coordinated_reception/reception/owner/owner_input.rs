pub struct OwnerAccessPermissionInput<'a, ValueId, ValuePassword, Access> {
    pub value_id: &'a ValueId,
    pub value_password: Option<&'a ValuePassword>,
    pub access: &'a Access
}

pub struct OwnerPasswordGeneratorInput<'a, OwnerId, OwnerPassword, ValueId, Access, Policy> {
    pub owner_id: &'a OwnerId,
    pub owner_password: &'a OwnerPassword,
    pub value_id: &'a ValueId,
    pub access: &'a Access,
    pub policy: &'a Policy
}