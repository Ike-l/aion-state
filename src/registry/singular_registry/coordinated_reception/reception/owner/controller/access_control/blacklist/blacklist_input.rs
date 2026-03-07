pub struct BlacklistAccess<'a, Id, Access, Password> {
    pub id: &'a Id,
    pub access: &'a Access,
    pub password: &'a Password
}

pub struct BlacklistAllow<Id, Access> {
    pub id: Id,
    pub access: Access
}

pub struct BlacklistRelease<'a, Id> {
    pub id: &'a Id
}