pub struct OwnerOwn<'a, Id, ResourceId, Password> {
    pub id: Id,
    pub resource_id: ResourceId,
    pub password: &'a Password
}

pub struct OwnerRegister<Id, Password> {
    pub id: Id,
    pub password: Password
}

pub struct OwnerRelease<'a, Id, Password, ResourceId> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: &'a ResourceId
}

pub struct OwnerUpdatePassword<'a, Id, Password> {
    pub id: &'a Id,
    pub old_password: &'a Password,
    pub new_password: Password
}

pub struct OwnerUnregister<'a, Id, Password> {
    pub id: &'a Id,
    pub password: &'a Password
}

pub struct OwnerAllow<'a, Id, Password, ResourceId, Access> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: ResourceId,
    pub access: Access
}

pub struct OwnerAccess<'a, Id, ResourceId, Access, Password> {
    pub id: Option<&'a Id>,
    pub resource_id: &'a ResourceId,
    pub access: &'a Access,
    pub password: Option<&'a Password>
}