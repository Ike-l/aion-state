pub enum BlacklistCheckAccessResult {
    Verification(bool)
}

impl BlacklistCheckAccessResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Verification(true))
    }
}

pub enum BlacklistAllowResult<Password> {
    Allow(Option<Password>)
}

pub enum BlacklistReleaseResult {
    Release(bool)
}

impl BlacklistReleaseResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Release(true))
    }
}

pub enum BlacklistUnallowResult {
    Unallow(bool)
}

pub enum BlacklistReleaseAllResult {
    Release(bool)
}