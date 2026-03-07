use crate::prelude::{AccessControlAccess, AccessControlAccessResult, AccessControlAllow, AccessControlBlacklistAllowResult, AccessControlRelease, AccessControlReleaseResult, AccessControlWhitelistAllowResult, Blacklist, BlacklistAccess, BlacklistAllow, BlacklistRelease, BlacklistStorage, Whitelist, WhitelistAccess, WhitelistAllow, WhitelistRelease, WhitelistStorage};

pub mod whitelist;
pub mod blacklist;

pub mod access_control_input;
pub mod access_control_result;

pub struct AccessControl<WS, BS> {
    whitelist: Whitelist<WS>,
    blacklist: Blacklist<BS>,
}

// semantic difference between `Whitelist` & `Blacklist` is simply that the blacklist takes a password whereas a whitelist doesn't
// whitelist is good because if a resource is `owned` it is automatically rejected unless it is allowed by the control

impl<
    WS: WhitelistStorage,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>
> AccessControl<WS, BS> {
    // `allow_blacklist`'s blacklist is linked with the whitelist incase in the future we want to link them in some way
    // splits the `allow` behaviour into 2 functions instead of the alternative- an enum Target::[Whitelist | Blacklist] 
    // this is the simpler approach. And allows a combo function later if we decide a reason for it

    /// (currently) allows a whitelisted resource to be blacklisted as well
    pub fn allow_blacklist(
        &mut self,
        AccessControlAllow {
            id, access
        }: AccessControlAllow<WS::Id, WS::Access>
    ) -> AccessControlBlacklistAllowResult<BS::Password> {
        AccessControlBlacklistAllowResult::Blacklist(self.blacklist.allow(BlacklistAllow { id, access } ))
    }

    /// (currently) allows a blacklisted resource to be whitelisted as well
    pub fn allow_whitelist(
        &mut self,
        AccessControlAllow {
            id, access
        }: AccessControlAllow<WS::Id, WS::Access>
    ) -> AccessControlWhitelistAllowResult {
        AccessControlWhitelistAllowResult::Whitelist(self.whitelist.allow(WhitelistAllow { id, access } ))
    }

    /// If a password is given it will check the blacklist
    /// 
    /// if a password is not given it will check the whitelist
    /// 
    /// Will not check the whitelist if the blacklist check fails
    pub fn access(
        &self,
        AccessControlAccess {
            id, access, password
        }: AccessControlAccess<'_, WS::Id, WS::Access, BS::Password>
    ) -> AccessControlAccessResult {
        match password {
            Some(password) => AccessControlAccessResult::Blacklist(self.blacklist.access(BlacklistAccess { id, access, password })),
            None => AccessControlAccessResult::Whitelist(self.whitelist.access(WhitelistAccess { id, access })),
        }
    }

    // takes away a single access for a list
    pub fn block(
        &mut self,
    ) {}

    // release all accesses from all lists
    pub fn release(
        &mut self,
        AccessControlRelease {
            id
        }: AccessControlRelease<'_, WS::Id>
    ) -> AccessControlReleaseResult {
        AccessControlReleaseResult::Lists((
            self.whitelist.release(WhitelistRelease { id }),
            self.blacklist.release(BlacklistRelease { id })
        ))
    }
}
