pub struct SingularRegistryRegister<Id, Password> {
    pub id: Id,
    pub password: Password
}

pub struct SingularRegistryUnregister<'a, Id, Password> {
    pub id: &'a Id,
    pub password: &'a Password
}

pub struct SingularRegistryUpdatePassword<'a, Id, Password> {
    pub id: &'a Id,
    pub old_password: &'a Password,
    pub new_password: Password
}

pub struct SingularRegistryOwn<'a, Id, Password, ResourceId> {
    pub id: Id,
    pub password: &'a Password,
    pub resource_id: ResourceId
}

pub struct SingularRegistryReleaseResource<'a, Id, Password, ResourceId> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: &'a ResourceId
}

pub struct SingularRegistryReleaseResourceAll<'a, Id, Password, ResourceId> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub inputs: Vec<&'a ResourceId>
}

pub struct SingularRegistryAllow<'a, Id, Password, ResourceId, Access> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: ResourceId,
    pub access: Access
}

pub struct SingularRegistryUnallow<'a, Id, Password, ResourceId, Access> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: &'a ResourceId,
    pub access: &'a Access
}

pub struct SingularRegistryCheckAccess<'a, Id, ResourceId, Access, Password> {
    pub id: Option<&'a Id>,
    pub resource_id: &'a ResourceId,
    pub access: &'a Access,
    pub password: Option<&'a Password>
}

pub struct SingularRegistryReleaseAccess<'a, ResourceId, Access> {
    pub resource_id: &'a ResourceId,
    pub access: &'a Access
}

pub struct SingularRegistryReservation<'a, Id, Password, ResourceId, Access> {
    pub id: Id,
    pub password: &'a Password,
    pub resource_id: ResourceId,
    pub access: Access
}