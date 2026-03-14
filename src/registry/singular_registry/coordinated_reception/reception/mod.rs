use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, Host, Owner, ReservationStorage, WhitelistStorage};

pub mod host;
pub mod owner;

pub mod reception_input;
pub mod reception_result;

/// Applies `Owner` semantics, then `Host` semantics
pub struct Reception<RS, AS, OS, WS, BS, CS> {
    owner: Owner<OS, WS, BS, CS>,
    host: Host<RS, AS>
}

impl<
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<ResourceId = BS::Id, Id = OS::Id>
> Reception<RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor
{
    pub fn register() {}
    pub fn unregister() {}
    pub fn update_password() {}
    pub fn own() {}
    pub fn release() {}
    pub fn allow_whitelist() {}
    pub fn allow_blacklist() {}
    pub fn check_access() {}
    pub fn block_whitelist() {}
    pub fn block_blacklist() {}
    pub fn release_all() {}
    pub fn reserve() {}
    pub fn unreserve() {}
}