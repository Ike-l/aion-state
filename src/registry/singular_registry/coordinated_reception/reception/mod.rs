use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, Host, Owner, OwnerRegister, ReceptionRegister, ReceptionRegisterResult, ReservationStorage, WhitelistStorage};

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
    // Owner Specific 

    // no host semantics needed
    /// Register a user for authentication
    pub fn register(
        &mut self,
        ReceptionRegister {
            id, password
        }: ReceptionRegister<OS::Id, OS::Password>
    ) -> ReceptionRegisterResult {
        ReceptionRegisterResult::Owner(self.owner.register(OwnerRegister { id, password }))
    }
    pub fn unregister() {}
    pub fn update_password() {}

    pub fn own() {}
    pub fn release_resource() {}
    pub fn release_resource_all() {}

    pub fn allow_whitelist() {}
    pub fn allow_blacklist() {}
    pub fn unallow_whitelist() {}
    pub fn unallow_blacklist() {}

    // Host Specific 
    pub fn check_access() {}
    pub fn release_access() {}
    pub fn record_access() {}

    pub fn reserve() {}
    pub fn unreserve() {}
}