pub struct HostCheckAccess<'a, ReserverId, AccessId, Access> {
    pub reserver_id: Option<&'a ReserverId>,
    pub access_id: &'a AccessId,
    pub access: &'a Access
}

pub struct HostRecordAccess<'a, ReserverId, AccessId, Access> {
    pub reserver_id: Option<&'a ReserverId>,
    pub access_id: AccessId,
    pub access: Access
}

pub struct HostReleaseAccess<'a, AccessId, Access> {
    pub access_id: &'a AccessId,
    pub access: &'a Access
}

pub struct HostReservation<ReserverId, AccessId, Access> {
    pub reserver_id: ReserverId,
    pub access_id: AccessId,
    pub access: Access
}

pub struct HostUnreserve<'a, ReserverId, AccessId, Access> {
    pub reserver_id: &'a ReserverId,
    pub access_id: &'a AccessId,
    pub access: &'a Access
}

pub struct HostDrainReservations<'a, ReserverId> {
    pub reserver_id: &'a ReserverId
}