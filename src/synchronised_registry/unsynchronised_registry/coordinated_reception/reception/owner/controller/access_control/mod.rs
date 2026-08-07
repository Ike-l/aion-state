use crate::prelude::{AccessControlCheckAccess, AccessControlCheckAccessResult, AccessControlAllow, AccessControlBlacklistAllowResult, AccessControlBlacklistUnallowResult, AccessControlUnallow, AccessControlRelease, AccessControlReleaseAllResult, AccessControlReleaseResult, AccessControlWhitelistAllowResult, AccessControlWhitelistUnallowResult, Blacklist, BlacklistCheckAccess, BlacklistAllow, BlacklistUnallow, BlacklistRelease, BlacklistStorage, Whitelist, WhitelistCheckAccess, WhitelistAllow, WhitelistUnallow, WhitelistRelease, WhitelistStorage, trace_function};

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
#[derive(Default, serde::Serialize, serde::Deserialize)]
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
        AccessControlCheckAccess {
            id, access, password
        }: &AccessControlCheckAccess<'_, WS::Id, WS::Access, BS::Password>
    ) -> AccessControlCheckAccessResult {
        trace_function!("AccessControl Check Access");

        let whitelist_result = self.whitelist.check_access(&WhitelistCheckAccess { id, access });
        match password {
            Some(password) => AccessControlCheckAccessResult::Lists { 
                whitelist: whitelist_result,
                blacklist: Some(self.blacklist.check_access(&BlacklistCheckAccess { id, access, password }))
            },
            None => AccessControlCheckAccessResult::Lists {
                whitelist: whitelist_result,
                blacklist: None
            }
        }
    }

    /// takes away a single access for a list
    /// 
    /// Passes through to `whitelist`
    pub fn unallow_whitelist(
        &mut self,
        AccessControlUnallow {
            id, access
        }: &AccessControlUnallow<'_, WS::Id, WS::Access>
    ) -> AccessControlWhitelistUnallowResult {
        trace_function!("AccessControl Unallow Whitelist");

        AccessControlWhitelistUnallowResult::Whitelist(self.whitelist.unallow(&WhitelistUnallow { id, access }))
    }

    /// Passes through to `blacklist`
    pub fn unallow_blacklist(
        &mut self,
        AccessControlUnallow {
            id, access
        }: &AccessControlUnallow<'_, BS::Id, BS::Access>
    ) -> AccessControlBlacklistUnallowResult {
        trace_function!("AccessControl Unallow Blacklist");

        AccessControlBlacklistUnallowResult::Blacklist(self.blacklist.unallow(&BlacklistUnallow { id, access }))
    }

    // release all accesses from all lists
    /// Passes through to `blacklist` & `whitelist` equally 
    /// 
    /// accesses are done in the order of the return 
    pub fn release(
        &mut self,
        AccessControlRelease {
            id
        }: &AccessControlRelease<'_, WS::Id>
    ) -> AccessControlReleaseResult {
        trace_function!("AccessControl Release");

        AccessControlReleaseResult::Lists((
            self.whitelist.release(&WhitelistRelease { id }),
            self.blacklist.release(&BlacklistRelease { id })
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
