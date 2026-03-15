use crate::prelude::{BlacklistCheckAccessResult, BlacklistAllowResult, BlacklistBlockResult, BlacklistReleaseAllResult, BlacklistReleaseResult, WhitelistCheckAccessResult, WhitelistAllowResult, WhitelistBlockResult, WhitelistReleaseAllResult, WhitelistReleaseResult};

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

pub enum AccessControlWhitelistBlockResult {
    Whitelist(WhitelistBlockResult)
}

pub enum AccessControlBlacklistBlockResult {
    Blacklist(BlacklistBlockResult)
}

pub enum AccessControlReleaseAllResult {
    Lists((
        WhitelistReleaseAllResult,
        BlacklistReleaseAllResult
    ))
}