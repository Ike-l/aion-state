use crate::prelude::{BlacklistAccess, BlacklistAccessResult, BlacklistAllow, BlacklistAllowResult, BlacklistBlock, BlacklistBlockResult, BlacklistRelease, BlacklistReleaseResult, BlacklistStorage};

pub mod blacklist_storage;

pub mod blacklist_input;
pub mod blacklist_result;

pub struct Blacklist<BS> {
    blacklist_storage: BS
}

impl<
    BS: BlacklistStorage
> Blacklist<BS> {
    pub fn allow(
        &mut self,
        BlacklistAllow {
            id, access
        }: BlacklistAllow<BS::Id, BS::Access>
    ) -> BlacklistAllowResult<BS::Password> {
        BlacklistAllowResult::Allow(self.blacklist_storage.allow(id, access))
    }

    pub fn access(
        &self,
        BlacklistAccess {
            id, access, password
        }: BlacklistAccess<'_, BS::Id, BS::Access, BS::Password>
    ) -> BlacklistAccessResult {
        BlacklistAccessResult::Verification(self.blacklist_storage.verify(id, access, password))
    }

    pub fn block(
        &mut self,
        BlacklistBlock {
            id, access   
        }: BlacklistBlock<'_, BS::Id, BS::Access>
    ) -> BlacklistBlockResult {
        BlacklistBlockResult::Block(self.blacklist_storage.block(id, access))
    }

    pub fn release(
        &mut self,
        BlacklistRelease {
            id
        }: BlacklistRelease<'_, BS::Id>
    ) -> BlacklistReleaseResult {
        BlacklistReleaseResult::Release(self.blacklist_storage.release(id))
    }
}