pub struct PasswordManagerAccessPermissionInput<'a, Password, Access> {
    pub password: Option<&'a Password>,
    pub access: &'a Access
}

pub struct PasswordGeneratorInput<'a, Access, Policy> {
    pub access: &'a Access,
    pub policy: &'a Policy,
}