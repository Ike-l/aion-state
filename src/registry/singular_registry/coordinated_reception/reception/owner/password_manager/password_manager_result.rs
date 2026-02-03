pub enum PasswordManagerAccessPermissionResult {
    Checked(bool)
}

impl PasswordManagerAccessPermissionResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Checked(true))
    }
}

pub enum PasswordGeneratorResult<Password> {
    Generated(Option<Password>),
}