pub struct AccessControlAccess<'a, Id, Access, Password> {
    pub id: &'a Id,
    pub access: &'a Access,
    pub password: Option<&'a Password>
}