pub struct OwnerOwn<'a, Id, ResourceId, Password> {
    pub id: Id,
    pub resource_id: ResourceId,
    pub password: &'a Password
}