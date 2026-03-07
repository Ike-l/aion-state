use crate::prelude::{BlacklistAccessResult, BlacklistAllowResult, WhitelistAccessResult, WhitelistAllowResult};

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