pub struct PermitsAccessInput<'a, AccessKey, Access> {
    pub access_key: &'a AccessKey,
    pub access: &'a Access,
}

pub struct RecordAccessInput<AccessKey, Access> {
    pub access_key: AccessKey,
    pub access: Access
}

pub struct RemoveAccessInput<'a, AccessKey, Access> {
    pub access_key: &'a AccessKey,
    pub access: &'a Access
}