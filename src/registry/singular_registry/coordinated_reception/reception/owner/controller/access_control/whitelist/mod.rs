use crate::prelude::{WhitelistAccess, WhitelistAccessResult, WhitelistAllow, WhitelistAllowResult, WhitelistStorage};

pub mod whitelist_storage;

pub mod whitelist_input;
pub mod whitelist_result;

/// Accesses are allowed without password
pub struct Whitelist<WS> {
    whitelist_storage: WS
}

impl<
    WS: WhitelistStorage
> Whitelist<WS> {
    pub fn allow(
        &self,
        WhitelistAllow {
            id, access
        }: WhitelistAllow<WS::Id, WS::Access>
    ) -> WhitelistAllowResult {
        WhitelistAllowResult::Allow(self.whitelist_storage.allow(id, access))
    }

    pub fn access(
        &self,
        WhitelistAccess {
            id, access,
        }: WhitelistAccess<'_, WS::Id, WS::Access>
    ) -> WhitelistAccessResult {
        WhitelistAccessResult::Allowed(self.whitelist_storage.verify(id, access))
    }
}