pub struct AuthenticateInput<'a, Id, Password> {
    pub id: &'a Id,
    pub password: &'a Password,
}