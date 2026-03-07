pub struct ResourceControlVerification<'a, Id, ResourceId> {
    pub id: &'a Id,
    pub resource_id: &'a ResourceId
}

pub struct ResourceControlRelease<'a, Id, ResourceId> {
    pub id: &'a Id,
    pub resource_id: &'a ResourceId
}