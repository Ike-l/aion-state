pub struct ReservationsAccessPermissionInput<'a, Reserver, AccessKey, Access> {
    pub reserver: Option<&'a Reserver>,
    pub access_key: &'a AccessKey,
    pub access: &'a Access
}

pub struct ReserveInput<Reserver, AccessKey, Access> {
    pub reserver: Reserver,
    pub access_key: AccessKey,
    pub access: Access,
}

pub struct UnreserveInput<'a, Reserver, AccessKey, Access> {
    pub reserver: &'a Reserver,
    pub access_key: &'a AccessKey,
    pub access: &'a Access
}