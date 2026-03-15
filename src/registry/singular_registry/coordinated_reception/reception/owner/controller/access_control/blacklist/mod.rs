use crate::prelude::{BlacklistCheckAccess, BlacklistCheckAccessResult, BlacklistAllow, BlacklistAllowResult, BlacklistBlock, BlacklistBlockResult, BlacklistRelease, BlacklistReleaseAllResult, BlacklistReleaseResult, BlacklistStorage, trace_function};

pub mod blacklist_storage;

pub mod blacklist_input;
pub mod blacklist_result;

/// Wraps `Blacklist Storage`
/// 
/// A `Blacklist` requires a `password` to permit an access
pub struct Blacklist<BS> {
    blacklist_storage: BS
}

impl<
    BS: BlacklistStorage
> Blacklist<BS> {
    /// Makes the `id` accessible with the matching `access`
    /// 
    /// Returns the password for this
    pub fn allow(
        &mut self,
        BlacklistAllow {
            id, access
        }: BlacklistAllow<BS::Id, BS::Access>
    ) -> BlacklistAllowResult<BS::Password> {
        trace_function!("Blacklist Allow");

        BlacklistAllowResult::Allow(self.blacklist_storage.allow(id, access))
    }

    /// Attempt to access the `id` with the `access` 
    /// 
    /// given the password is the one returned by the corresponding `allow`
    pub fn check_access(
        &self,
        BlacklistCheckAccess {
            id, access, password
        }: BlacklistCheckAccess<'_, BS::Id, BS::Access, BS::Password>
    ) -> BlacklistCheckAccessResult {
        trace_function!("Blacklist Check Access");

        BlacklistCheckAccessResult::Verification(self.blacklist_storage.check_access(id, access, password))
    }

    /// Attempt to un-allow
    /// 
    /// Passes directly to `blacklist_storage`
    pub fn block(
        &mut self,
        BlacklistBlock {
            id, access   
        }: BlacklistBlock<'_, BS::Id, BS::Access>
    ) -> BlacklistBlockResult {
        trace_function!("Blacklist Block");

        BlacklistBlockResult::Block(self.blacklist_storage.block(id, access))
    }

    /// Attempts to un-allow all `access` corresponding to `id`
    pub fn release(
        &mut self,
        BlacklistRelease {
            id
        }: BlacklistRelease<'_, BS::Id>
    ) -> BlacklistReleaseResult {
        trace_function!("Blacklist Release");

        BlacklistReleaseResult::Release(self.blacklist_storage.release(id))
    }

    /// release all accesses corresponding to the `ids` in `inputs`
    /// 
    /// use this over iterating over `release` if the implementator of `BlacklistStorage` has additional semantics on their `release_all`
    pub fn release_all<'a>(
        &mut self,
        inputs: impl Iterator<Item = BlacklistRelease<'a, BS::Id>>
    ) -> BlacklistReleaseAllResult where <BS as BlacklistStorage>::Id: 'a {
        trace_function!("Blacklist Release All");

        BlacklistReleaseAllResult::Release(self.blacklist_storage.release_all(inputs.map(|BlacklistRelease { id }| id)))
    }
}