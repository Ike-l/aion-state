pub struct OwnerOwn<'a, Id, ResourceId, Password> {
    pub id: Id,
    pub resource_id: ResourceId,
    pub password: &'a Password
}

pub struct OwnerRegister<Id, Password> {
    pub id: Id,
    pub password: Password
}