use crate::prelude::{WhitelistAccess, WhitelistAccessResult, WhitelistAllow, WhitelistAllowResult, WhitelistBlock, WhitelistBlockResult, WhitelistRelease, WhitelistReleaseAllResult, WhitelistReleaseResult, WhitelistStorage, trace_function};

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
        WhitelistAccess {
            id, access,
        }: WhitelistAccess<'_, WS::Id, WS::Access>
    ) -> WhitelistAccessResult {
        trace_function!("Whitelist Check Access");

        WhitelistAccessResult::Allowed(self.whitelist_storage.check_access(id, access))
    }

    // opposite to allow
    /// Passes through to `whitelist_storage`
    pub fn block(
        &mut self,
        WhitelistBlock {
            id, access
        }: WhitelistBlock<'_, WS::Id, WS::Access>
    ) -> WhitelistBlockResult {
        trace_function!("Whitelist Block");

        WhitelistBlockResult::Block(self.whitelist_storage.block(id, access))
    }

    // remove all allowances
    /// Passes through to `whitelist_storage`
    pub fn release(
        &mut self,
        WhitelistRelease {
            id
        }: WhitelistRelease<'_, WS::Id>
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