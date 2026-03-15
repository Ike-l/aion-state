pub struct ResourceControlCheckResourceOwner<'a, Id, ResourceId> {
    pub id: &'a Id,
    pub resource_id: &'a ResourceId
}

pub struct ResourceControlRelease<'a, Id, ResourceId> {
    pub id: &'a Id,
    pub resource_id: &'a ResourceId
}

pub struct ResourceControlOwn<Id, ResourceId> {
    pub id: Id,
    pub resource_id: ResourceId
}

pub struct ResourceControlReleaseId<'a, Id> {
    pub id: &'a Id
}