pub struct AuthenticateInput<'a, OwnerId, OwnerKey> {
    pub owner_id: &'a OwnerId,
    pub owner_key: &'a OwnerKey
}