pub struct Authentication<'a, Id, Password> {
    pub id: &'a Id,
    pub password: &'a Password,
}

pub struct AuthenticateRegister<Id, Password> {
    pub id: Id,
    pub password: Password
}

pub struct AuthenticateUpdatePassword<'a, Id, Password> {
    pub id: &'a Id,
    pub new_password: Password
}

pub struct AuthenticateUnregister<'a, Id> {
    pub id: &'a Id
}