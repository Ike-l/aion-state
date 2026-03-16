use crate::prelude::{WhitelistCheckAccess, WhitelistCheckAccessResult, WhitelistAllow, WhitelistAllowResult, WhitelistUnallow, WhitelistUnallowResult, WhitelistRelease, WhitelistReleaseAllResult, WhitelistReleaseResult, WhitelistStorage, trace_function};

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
    /// Passes through to `whitelist_storage`
    pub fn allow(
        &mut self,
        WhitelistAllow {
            id, access
        }: WhitelistAllow<WS::Id, WS::Access>
    ) -> WhitelistAllowResult {
        trace_function!("Whitelist Allow");

        WhitelistAllowResult::Allow(self.whitelist_storage.allow(id, access))
    }

    /// Passes through to `whitelist_storage`
    pub fn check_access(
        &self,
        WhitelistCheckAccess {
            id, access,
        }: &WhitelistCheckAccess<'_, WS::Id, WS::Access>
    ) -> WhitelistCheckAccessResult {
        trace_function!("Whitelist Check Access");

        WhitelistCheckAccessResult::Allowed(self.whitelist_storage.check_access(id, access))
    }

    // opposite to allow
    /// Passes through to `whitelist_storage`
    pub fn unallow(
        &mut self,
        WhitelistUnallow {
            id, access
        }: &WhitelistUnallow<'_, WS::Id, WS::Access>
    ) -> WhitelistUnallowResult {
        trace_function!("Whitelist Unallow");

        WhitelistUnallowResult::Unallow(self.whitelist_storage.unallow(id, access))
    }

    // remove all allowances
    /// Passes through to `whitelist_storage`
    pub fn release(
        &mut self,
        WhitelistRelease {
            id
        }: &WhitelistRelease<'_, WS::Id>
    ) -> WhitelistReleaseResult {
        trace_function!("Whitelist Release");

        WhitelistReleaseResult::Release(self.whitelist_storage.release(id))
    }

    /// release all `ids` in `inputs`
    /// 
    /// use this over iterating over `release` if the implementator of `WhitelistStorage` has additional semantics on their `release_all`
    pub fn release_all<'a>(
        &mut self,
        inputs: impl Iterator<Item = WhitelistRelease<'a, WS::Id>>
    ) -> WhitelistReleaseAllResult where <WS as WhitelistStorage>::Id: 'a {
        trace_function!("Whitelist Release All");

        WhitelistReleaseAllResult::Release(self.whitelist_storage.release_all(inputs.map(|WhitelistRelease { id }| id)))
    }
}