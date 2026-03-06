pub enum BlacklistAccessResult {
    Verification(bool)
}

pub enum BlacklistAllowResult<Password> {
    Allow(Option<Password>)
}