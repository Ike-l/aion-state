pub struct ReservationMapAccessPermissionInput<'a, Reserver, AccessKey, Access> {
    pub reserver: Option<&'a Reserver>,
    pub access_key: &'a AccessKey,
    pub access: &'a Access
}