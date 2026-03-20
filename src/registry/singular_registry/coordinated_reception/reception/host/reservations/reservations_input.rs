pub struct ReservationsCheckAccess<'a, ReserverId, AccessId, Access> {
    pub reserver_id: Option<&'a ReserverId>,
    pub access_id: &'a AccessId,
    pub access: &'a Access
}

pub struct ReservationsReservation<ReserverId, AccessId, Access> {
    pub reserver_id: ReserverId,
    pub access_id: AccessId,
    pub access: Access,
}

pub struct ReservationsUnreserve<'a, ReserverId, AccessId, Access> {
    pub reserver_id: &'a ReserverId,
    pub access_id: &'a AccessId,
    pub access: &'a Access
}

pub struct ReservationsDrainReservations<'a, ReserverId> {
    pub reserver_id: &'a ReserverId
}