pub struct ReceptionAccessPermissionInput <'a, ReserverId, AccessId, Access, OwnerId, OwnerPassword, ValuePassword> {
    pub reserver_id: Option<&'a ReserverId>,
    pub access_id: &'a AccessId,
    pub access: &'a Access,
    pub owner_credentials: Option<(&'a OwnerId, &'a OwnerPassword)>,
    pub value_password: Option<&'a ValuePassword>
}