pub struct PasswordManagerAccessPermissionInput<'a, Password, Access> {
    pub password: &'a Password,
    pub access: &'a Access
}

pub struct PasswordGeneratorInput<'a, Access> {
    pub access: &'a Access
}