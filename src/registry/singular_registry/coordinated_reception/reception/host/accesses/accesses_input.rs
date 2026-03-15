pub struct AccessesCheckAccess<'a, AccessId, Access> {
    pub access_id: &'a AccessId,
    pub access: &'a Access,
}

pub struct AccessesRecordAccess<AccessId, Access> {
    pub access_id: AccessId,
    pub access: Access
}

pub struct AccessesRelease<'a, AccessId, Access> {
    pub access_id: &'a AccessId,
    pub access: &'a Access
}