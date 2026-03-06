use crate::prelude::{BlacklistAccessResult, WhitelistAccessResult};

pub enum AccessControlAccessResult {
    Whitelist(WhitelistAccessResult),
    Blacklist(BlacklistAccessResult)
}