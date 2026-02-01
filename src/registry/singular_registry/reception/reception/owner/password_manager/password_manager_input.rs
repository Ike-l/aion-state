pub struct PasswordManagerAccessPermissionInput<'a, Password, Access> {
    pub password: &'a Password,
    pub access: &'a Access
}