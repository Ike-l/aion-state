pub struct ReceptionAccessPermissionInput <'a, Reserver, AccessKey, Access, OwnerId, OwnerKey> {
    pub reserver: Option<&'a Reserver>,
    pub access_key: &'a AccessKey,
    pub access: &'a Access,
    pub owner_credentials: Option<(&'a OwnerId, &'a OwnerKey)>
}