use crate::prelude::{BlacklistAccessResult, BlacklistAllowResult, BlacklistBlockResult, BlacklistReleaseResult, WhitelistAccessResult, WhitelistAllowResult, WhitelistBlockResult, WhitelistReleaseResult};

pub enum AccessControlAccessResult {
    Whitelist(WhitelistAccessResult),
    Blacklist(BlacklistAccessResult)
}

pub enum AccessControlBlacklistAllowResult<Password> {
    Blacklist(BlacklistAllowResult<Password>)
}

pub enum AccessControlWhitelistAllowResult {
    Whitelist(WhitelistAllowResult)
}

pub enum AccessControlReleaseResult {
    Lists((WhitelistReleaseResult, BlacklistReleaseResult))
}

impl AccessControlReleaseResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Lists((w, b)) => w.ok() && b.ok(),
        }
    }
}

pub enum AccessControlWhitelistBlockResult {
    Whitelist(WhitelistBlockResult)
}

pub enum AccessControlBlacklistBlockResult {
    Blacklist(BlacklistBlockResult)
}