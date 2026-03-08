pub enum WhitelistAccessResult {
    Allowed(bool)
}

impl WhitelistAccessResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Allowed(true))
    }
}

pub enum WhitelistAllowResult {
    Allow(bool)
}

pub enum WhitelistReleaseResult {
    Release(bool)
}

impl WhitelistReleaseResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Release(true))
    }
}

pub enum WhitelistBlockResult {
    Block(bool)
}