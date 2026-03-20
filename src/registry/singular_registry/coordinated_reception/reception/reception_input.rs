pub struct ReceptionRegister<Id, Password> {
    pub id: Id,
    pub password: Password
}

pub struct ReceptionUnregister<'a, Id, Password> {
    pub id: &'a Id,
    pub password: &'a Password
}

pub struct ReceptionUpdatePassword<'a, Id, Password> {
    pub id: &'a Id,
    pub old_password: &'a Password,
    pub new_password: Password,
}

pub struct ReceptionOwn<'a, Id, Password, ResourceId> {
    pub id: Id,
    pub password: &'a Password,
    pub resource_id: ResourceId
}

pub struct ReceptionReleaseResource<'a, Id, Password, ResourceId> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: &'a ResourceId
}

pub struct ReceptionAllow<'a, Id, Password, ResourceId, Access> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: ResourceId,
    pub access: Access
}

pub struct ReceptionUnallow<'a, Id, Password, ResourceId, Access> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: &'a ResourceId,
    pub access: &'a Access
}

pub struct ReceptionReleaseResourceAll<'a, Id, Password, ResourceId> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub inputs: Vec<&'a ResourceId>,
}

pub struct ReceptionCheckAccess<'a, Id, ResourceId, Access, Password> {
    pub id: Option<&'a Id>,
    pub resource_id: &'a ResourceId,
    pub access: &'a Access,
    pub password: Option<&'a Password>
}

pub struct ReceptionReleaseAccess<'a, ResourceId, Access> {
    pub resource_id: &'a ResourceId,
    pub access: &'a Access
}

pub struct ReceptionRecordAccess<'a, Id, ResourceId, Access> {
    pub id: Option<&'a Id>,
    pub resource_id: ResourceId,
    pub access: Access
}