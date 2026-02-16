pub struct ReceptionAccessPermissionInput<'a, ReserverId, AccessId, Access, ValuePassword> {
    pub reserver_id: Option<&'a ReserverId>,
    pub access_id: &'a AccessId,
    pub access: &'a Access,
    pub value_password: Option<&'a ValuePassword>
}

pub struct ReceptionPasswordGeneratorInput<'a, OwnerId, OwnerPassword, ValueId, Access, Policy> {
    pub owner_id: &'a OwnerId,
    pub owner_password: &'a OwnerPassword,
    pub value_id: &'a ValueId,
    pub access: &'a Access,
    pub policy: &'a Policy
}