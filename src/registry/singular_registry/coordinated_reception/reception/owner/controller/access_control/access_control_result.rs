use crate::prelude::{BlacklistAccessResult, BlacklistAllowResult, BlacklistReleaseResult, WhitelistAccessResult, WhitelistAllowResult, WhitelistReleaseResult};

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
            _ => false
        }
    }
}