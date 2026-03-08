pub struct ControllerOwn<Id, ResourceId> {
    pub id: Id,
    pub resource_id: ResourceId
}

pub struct ControllerRelease<'a, Id, ResourceId> {
    pub id: &'a Id,
    pub resource_id: &'a ResourceId
}

pub struct ControllerAllow<'a, Id, ResourceId, Access> {
    pub id: &'a Id,
    pub resource_id: ResourceId,
    pub access: Access
}

pub struct ControllerAccess<'a, Id, Access, Password> {
    pub id: &'a Id,
    pub access: &'a Access,
    pub password: Option<&'a Password>
}