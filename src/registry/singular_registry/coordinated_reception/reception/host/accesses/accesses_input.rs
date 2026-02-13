pub struct PermitsAccessInput<'a, AccessId, Access> {
    pub access_id: &'a AccessId,
    pub access: &'a Access,
}

pub struct RecordAccessInput<AccessId, Access> {
    pub access_id: AccessId,
    pub access: Access
}

pub struct RemoveAccessInput<'a, AccessId, Access> {
    pub access_id: &'a AccessId,
    pub access: &'a Access
}