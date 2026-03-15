use crate::prelude::{BlacklistCheckAccessResult, BlacklistAllowResult, BlacklistUnallowResult, BlacklistReleaseAllResult, BlacklistReleaseResult, WhitelistCheckAccessResult, WhitelistAllowResult, WhitelistUnallowResult, WhitelistReleaseAllResult, WhitelistReleaseResult};

pub enum AccessControlCheckAccessResult {
    Whitelist(WhitelistCheckAccessResult),
    Blacklist(BlacklistCheckAccessResult)
}

impl AccessControlCheckAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            AccessControlCheckAccessResult::Whitelist(whitelist_access_result) => whitelist_access_result.ok(),
            AccessControlCheckAccessResult::Blacklist(blacklist_access_result) => blacklist_access_result.ok(),
        }
    }
}

pub enum AccessControlBlacklistAllowResult<Password> {
    Blacklist(BlacklistAllowResult<Password>)
}

pub enum AccessControlWhitelistAllowResult {
    Whitelist(WhitelistAllowResult)
}

pub enum AccessControlReleaseResult {
    Lists((
        WhitelistReleaseResult, 
        BlacklistReleaseResult
    ))
}

impl AccessControlReleaseResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Lists((w, b)) => w.ok() && b.ok(),
        }
    }
}

pub enum AccessControlWhitelistUnallowResult {
    Whitelist(WhitelistUnallowResult)
}

pub enum AccessControlBlacklistUnallowResult {
    Blacklist(BlacklistUnallowResult)
}

pub enum AccessControlReleaseAllResult {
    Lists((
        WhitelistReleaseAllResult,
        BlacklistReleaseAllResult
    ))
}