use crate::prelude::{BlacklistAccess, BlacklistAccessResult, BlacklistAllow, BlacklistAllowResult, BlacklistBlock, BlacklistBlockResult, BlacklistRelease, BlacklistReleaseResult, BlacklistStorage};

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
        BlacklistAllowResult::Allow(self.blacklist_storage.allow(id, access))
    }

    /// Attempt to access the `id` with the `access` 
    /// 
    /// given the password is the one returned by the corresponding `allow`
    pub fn access(
        &self,
        BlacklistAccess {
            id, access, password
        }: BlacklistAccess<'_, BS::Id, BS::Access, BS::Password>
    ) -> BlacklistAccessResult {
        BlacklistAccessResult::Verification(self.blacklist_storage.verify(id, access, password))
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
        BlacklistBlockResult::Block(self.blacklist_storage.block(id, access))
    }

    /// Attempts to un-allow all `access` corresponding to `id`
    pub fn release(
        &mut self,
        BlacklistRelease {
            id
        }: BlacklistRelease<'_, BS::Id>
    ) -> BlacklistReleaseResult {
        BlacklistReleaseResult::Release(self.blacklist_storage.release(id))
    }
}