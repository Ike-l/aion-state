use crate::prelude::{BlacklistCheckAccessResult, BlacklistAllowResult, BlacklistUnallowResult, BlacklistReleaseAllResult, BlacklistReleaseResult, WhitelistCheckAccessResult, WhitelistAllowResult, WhitelistUnallowResult, WhitelistReleaseAllResult, WhitelistReleaseResult};

#[derive(Debug)]
pub enum AccessControlCheckAccessResult {
    Lists {
        whitelist: WhitelistCheckAccessResult,
        blacklist: Option<BlacklistCheckAccessResult>,
    }
}

impl AccessControlCheckAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            AccessControlCheckAccessResult::Lists { whitelist, blacklist } => whitelist.ok() || blacklist.as_ref().is_some_and(|blacklist_result| blacklist_result.ok()),
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