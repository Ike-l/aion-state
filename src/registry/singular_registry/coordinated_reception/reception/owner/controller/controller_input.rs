pub struct ControllerOwn<Id, ResourceId> {
    pub id: Id,
    pub resource_id: ResourceId
}

pub struct ControllerRelease<'a, Id, ResourceId> {
    pub id: &'a Id,
    pub resource_id: &'a ResourceId
}