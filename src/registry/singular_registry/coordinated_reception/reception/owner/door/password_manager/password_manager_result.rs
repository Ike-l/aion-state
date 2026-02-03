pub enum PasswordCheckResult {
    Checked(bool)
}

impl PasswordCheckResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Checked(true))
    }
}

pub enum PasswordGeneratorResult<Password> {
    Generated(Option<Password>),
}