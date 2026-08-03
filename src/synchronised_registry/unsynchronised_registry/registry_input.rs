pub struct RegistryRegister<Id, Password> {
    pub id: Id,
    pub password: Password
}

pub struct RegistryUnregister<'a, Id, Password> {
    pub id: &'a Id,
    pub password: &'a Password
}

pub struct RegistryUpdatePassword<'a, Id, Password> {
    pub id: &'a Id,
    pub old_password: &'a Password,
    pub new_password: Password
}

pub struct RegistryOwn<'a, Id, Password, ResourceId> {
    pub id: Id,
    pub password: &'a Password,
    pub resource_id: ResourceId
}

pub struct RegistryReleaseResource<'a, Id, Password, ResourceId> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: &'a ResourceId
}

pub struct RegistryReleaseResourceAll<'a, Id, Password, ResourceId> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub inputs: Vec<&'a ResourceId>
}

pub struct RegistryAllow<'a, Id, Password, ResourceId, Access> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: ResourceId,
    pub access: Access
}

pub struct RegistryUnallow<'a, Id, Password, ResourceId, Access> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: &'a ResourceId,
    pub access: &'a Access
}

pub struct RegistryCheckAccess<'a, Id, IdPassword, ResourceId, Access, Password> {
    pub user_details: Option<(&'a Id, &'a IdPassword)>,
    pub resource_id: &'a ResourceId,
    pub access: &'a Access,
    pub password: Option<&'a Password>
}

pub struct RegistryReleaseAccess<'a, ResourceId, Access> {
    pub resource_id: &'a ResourceId,
    pub access: &'a Access
}

#[cfg(feature = "releaser")]
pub struct RegistryReleasingReleaseAccess<ResourceId, Access> {
    pub resource_id: ResourceId,
    pub access: Access
}

pub struct RegistryReservation<'a, Id, IdPassword, ResourceId, Access, Password> {
    pub id: Id,
    pub id_password: &'a IdPassword,
    pub resource_id: ResourceId,
    pub access: Access,
    pub password: Option<&'a Password>
}

pub struct RegistryUnreserve<'a, Id, Password, ResourceId, Access> {
    pub id: &'a Id,
    pub password: &'a Password,
    pub resource_id: &'a ResourceId,
    pub access: &'a Access
}

pub struct RegistryDrainReservations<'a, Id, Password> {
    pub id: &'a Id,
    pub password: &'a Password
}

pub struct RegistryAcquireAccess<'a, Id, IdPassword, ResourceId, Access, Password> {
    pub user_details: Option<(&'a Id, &'a IdPassword)>,
    pub resource_id: ResourceId,
    pub access: Access,
    pub password: Option<&'a Password>
}

#[cfg(any(feature = "releaser", feature = "notifier"))]
#[derive(Clone)]
pub struct RegistryOwnedAcquireAccess<Id, IdPassword, ResourceId, Access, Password> {
    pub user_details: Option<(Id, IdPassword)>,
    pub resource_id: ResourceId,
    pub access: Access,
    pub password: Option<Password>
}


pub struct RegistrySaferReplacement<'a, Id, IdPassword, Access, ResourceId, Resource, Password> {
    pub user_details: Option<(&'a Id, &'a IdPassword)>,
    pub access: &'a Access,
    pub resource_id: ResourceId,
    pub resource: Option<Resource>,
    pub password: Option<&'a Password>,
}

pub struct RegistryContainsResource<'a, ResourceId> {
    pub resource_id: &'a ResourceId
}