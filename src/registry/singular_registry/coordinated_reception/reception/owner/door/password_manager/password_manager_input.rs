pub struct PasswordManagerAccessPermissionInput<'a, ValuePassword, Access> {
    pub value_password: Option<&'a ValuePassword>,
    pub access: &'a Access
}

pub struct PasswordGeneratorInput<'a, Access, Policy> {
    pub access: &'a Access,
    pub policy: &'a Policy,
}