pub struct AuthenticateInput<'a, OwnerId, OwnerPassword, ValueId> {
    pub owner_id: &'a OwnerId,
    pub owner_password: &'a OwnerPassword,
    pub value_id: &'a ValueId
}