use crate::prelude::{AccessControlAccess, AccessControlAccessResult, AccessControlAllow, AccessControlBlacklistAllowResult, AccessControlBlacklistBlockResult, AccessControlBlock, AccessControlRelease, AccessControlReleaseAllResult, AccessControlReleaseResult, AccessControlWhitelistAllowResult, AccessControlWhitelistBlockResult, Blacklist, BlacklistAccess, BlacklistAllow, BlacklistBlock, BlacklistRelease, BlacklistStorage, Whitelist, WhitelistAccess, WhitelistAllow, WhitelistBlock, WhitelistRelease, WhitelistStorage, trace_function};

pub mod whitelist;
pub mod blacklist;

pub mod access_control_input;
pub mod access_control_result;

/// Whitelist & Blacklist are unordered
/// 
/// AccessControl acts as the single point of communication & future modification center for "owner pin holes"
///
/// Semantic difference between `Whitelist` & `Blacklist` is simply that the blacklist takes a password whereas a whitelist doesn't
///
/// Whitelist is good because if a resource is `owned` it is automatically rejected unless it is allowed by the control
pub struct AccessControl<WS, BS> {
    whitelist: Whitelist<WS>,
    blacklist: Blacklist<BS>,
}

/// `allow_blacklist`'s blacklist is linked with the whitelist incase in the future we want to link them in some way
/// splits the `allow` behaviour into 2 functions instead of the alternative- an enum Target::[Whitelist | Blacklist] 
/// this is the simpler approach. And allows a combo function later if we decide a reason for it
impl<
    WS: WhitelistStorage,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>
> AccessControl<WS, BS> {
    /// (currently) allows a whitelisted resource to be blacklisted as well
    /// 
    /// Passes through to `blacklist`
    pub fn allow_blacklist(
        &mut self,
        AccessControlAllow {
            id, access
        }: AccessControlAllow<WS::Id, WS::Access>
    ) -> AccessControlBlacklistAllowResult<BS::Password> {
        trace_function!("AccessControl Allow Blacklist");

        AccessControlBlacklistAllowResult::Blacklist(self.blacklist.allow(BlacklistAllow { id, access } ))
    }

    /// (currently) allows a whitelisted resource to be blacklisted as well
    /// 
    /// Passes through to `whitelist`
    pub fn allow_whitelist(
        &mut self,
        AccessControlAllow {
            id, access
        }: AccessControlAllow<WS::Id, WS::Access>
    ) -> AccessControlWhitelistAllowResult {
        trace_function!("AccessControl Allow Whitelist");

        AccessControlWhitelistAllowResult::Whitelist(self.whitelist.allow(WhitelistAllow { id, access } ))
    }

    /// If a password is given it will check the blacklist
    /// 
    /// if a password is not given it will check the whitelist
    /// 
    /// Does not (currently) check the whitelist if the blacklist check fails
    pub fn check_access(
        &self,
        AccessControlAccess {
            id, access, password
        }: AccessControlAccess<'_, WS::Id, WS::Access, BS::Password>
    ) -> AccessControlAccessResult {
        trace_function!("AccessControl Check Access");

        match password {
            Some(password) => AccessControlAccessResult::Blacklist(self.blacklist.check_access(BlacklistAccess { id, access, password })),
            None => AccessControlAccessResult::Whitelist(self.whitelist.check_access(WhitelistAccess { id, access })),
        }
    }

    /// takes away a single access for a list
    /// 
    /// Passes through to `whitelist`
    pub fn block_whitelist(
        &mut self,
        AccessControlBlock {
            id, access
        }: AccessControlBlock<'_, WS::Id, WS::Access>
    ) -> AccessControlWhitelistBlockResult {
        trace_function!("AccessControl Block Whitelist");

        AccessControlWhitelistBlockResult::Whitelist(self.whitelist.block(WhitelistBlock { id, access }))
    }

    /// Passes through to `blacklist`
    pub fn block_blacklist(
        &mut self,
        AccessControlBlock {
            id, access
        }: AccessControlBlock<'_, BS::Id, BS::Access>
    ) -> AccessControlBlacklistBlockResult {
        trace_function!("AccessControl Block Blacklist");

        AccessControlBlacklistBlockResult::Blacklist(self.blacklist.block(BlacklistBlock { id, access }))
    }

    // release all accesses from all lists
    /// Passes through to `blacklist` & `whitelist` equally 
    /// 
    /// accesses are done in the order of the return 
    pub fn release(
        &mut self,
        AccessControlRelease {
            id
        }: AccessControlRelease<'_, WS::Id>
    ) -> AccessControlReleaseResult {
        trace_function!("AccessControl Release");

        AccessControlReleaseResult::Lists((
            self.whitelist.release(WhitelistRelease { id }),
            self.blacklist.release(BlacklistRelease { id })
        ))
    }

    /// Passes through to `whitelist` & `blacklist` 
    /// 
    /// The order should not be considered when analysing the semantics of the function 
    /// 
    /// So if either `whitelist` or `blacklist` panics the progress of the other is unknown
    /// 
    /// Semantics of `release_all` follow from the implementation of `whitelist_storage`
    /// 
    /// So it could be better than just iterating over `release`
    pub fn release_all(
        &mut self,
        inputs: Vec<AccessControlRelease<'_, WS::Id>>
    ) -> AccessControlReleaseAllResult {
        trace_function!("AccessControl Release All");

        let whitelist_input = inputs
            .iter()
            .map(
                |AccessControlRelease { id }| 
                    WhitelistRelease { id: *id }
                );
                
        let blacklist_input = inputs
            .iter()
            .map(
                |AccessControlRelease { id }| 
                    BlacklistRelease { id: *id }
                );
                
        AccessControlReleaseAllResult::Lists((
            self.whitelist.release_all(whitelist_input),
            self.blacklist.release_all(blacklist_input)
        ))
    }
}
