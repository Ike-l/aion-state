pub struct ReceptionAccessPermissionInput <'a, ReserverId, AccessId, Access, ValuePassword> {
    pub reserver_id: Option<&'a ReserverId>,
    pub access_id: &'a AccessId,
    pub access: &'a Access,
    pub value_password: Option<&'a ValuePassword>
}