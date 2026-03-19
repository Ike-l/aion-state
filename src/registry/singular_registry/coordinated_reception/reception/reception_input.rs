pub struct ReceptionRegister<Id, Password> {
    pub id: Id,
    pub password: Password
}

pub struct ReceptionUnregister<'a, Id, Password> {
    pub id: &'a Id,
    pub password: &'a Password
}

pub struct ReceptionUpdatePassword<'a, Id, Password> {
    pub id: &'a Id,
    pub old_password: &'a Password,
    pub new_password: Password,
}