pub struct AccessControlCheckAccess<'a, Id, Access, Password> {
    pub id: &'a Id,
    pub access: &'a Access,
    pub password: Option<&'a Password>
}

pub struct AccessControlAllow<Id, Access> {
    pub id: Id,
    pub access: Access
}

pub struct AccessControlRelease<'a, Id> {
    pub id: &'a Id
}

pub struct AccessControlBlock<'a, Id, Access> {
    pub id: &'a Id,
    pub access: &'a Access
}