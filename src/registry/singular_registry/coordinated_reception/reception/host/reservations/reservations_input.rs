pub struct ReservationsAccessPermissionInput<'a, ReserverId, AccessId, Access> {
    pub reserver_id: Option<&'a ReserverId>,
    pub access_id: &'a AccessId,
    pub access: &'a Access
}

pub struct ReserveInput<ReserverId, AccessId, Access> {
    pub reserver_id: ReserverId,
    pub access_id: AccessId,
    pub access: Access,
}

pub struct UnreserveInput<'a, ReserverId, AccessId, Access> {
    pub reserver_id: &'a ReserverId,
    pub access_id: &'a AccessId,
    pub access: &'a Access
}