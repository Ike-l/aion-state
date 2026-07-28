pub struct ControllerOwn<Id, ResourceId> {
    pub id: Id,
    pub resource_id: ResourceId
}

pub struct ControllerReleaseResource<'a, Id, ResourceId> {
    pub id: &'a Id,
    pub resource_id: &'a ResourceId
}

pub struct ControllerAllow<'a, Id, ResourceId, Access> {
    pub id: &'a Id,
    pub resource_id: ResourceId,
    pub access: Access
}

pub struct ControllerCheckAccess<'a, Id, ResourceId, Access, Password> {
    pub id: Option<&'a Id>,
    pub resource_id: &'a ResourceId,
    pub access: &'a Access,
    pub password: Option<&'a Password>
}

pub struct ControllerReleaseId<'a, Id> {
    pub id: &'a Id
}

pub struct ControllerUnallow<'a, Id, ResourceId, Access> {
    pub id: &'a Id,
    pub resource_id: &'a ResourceId,
    pub access: &'a Access
}

pub struct ControllerCheckOwner<'a, Id, ResourceId> {
    pub id: &'a Id,
    pub resource_id: &'a ResourceId
}

