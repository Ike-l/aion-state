pub struct HostAccessPermissionInput<'a, Reserver, AccessKey, Access> {
    pub reserver: Option<&'a Reserver>,
    pub access_key: &'a AccessKey,
    pub access: &'a Access
}

pub struct HostRecordAccessInput<'a, Reserver, AccessKey, Access> {
    pub reserver: Option<&'a Reserver>,
    pub access_key: AccessKey,
    pub access: Access
}