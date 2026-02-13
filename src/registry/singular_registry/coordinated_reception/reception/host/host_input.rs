pub struct HostAccessPermissionInput<'a, ReserverId, AccessId, Access> {
    pub reserver_id: Option<&'a ReserverId>,
    pub access_id: &'a AccessId,
    pub access: &'a Access
}

pub struct HostRecordAccessInput<'a, ReserverId, AccessId, Access> {
    pub reserver_id: Option<&'a ReserverId>,
    pub access_id: AccessId,
    pub access: Access
}