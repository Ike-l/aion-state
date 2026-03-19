use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, Host, Owner, OwnerOwn, OwnerRegister, OwnerUnregister, OwnerUpdatePassword, ReceptionOwn, ReceptionOwnResult, ReceptionRegister, ReceptionRegisterResult, ReceptionUnregister, ReceptionUnregisterResult, ReceptionUpdatePassword, ReceptionUpdatePasswordResult, ReservationStorage, WhitelistStorage, trace_function};

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
        trace_function!("Reception Register");

        ReceptionRegisterResult::Owner(self.owner.register(OwnerRegister { id, password }))
    }
    
    // does this need to release ReserverId ownership?
    // ^ would need a method to "remove" the access map associated with the reserver map
    pub fn unregister(
        &mut self,
        ReceptionUnregister {
            id, password
        }: ReceptionUnregister<'_, OS::Id, OS::Password>
    ) -> ReceptionUnregisterResult {
        trace_function!("Reception Unregister");
        
        ReceptionUnregisterResult::Owner(self.owner.unregister(&OwnerUnregister { id, password }))
    }
    
    pub fn update_password(
        &mut self,
        ReceptionUpdatePassword {
            id, old_password, new_password
        }: ReceptionUpdatePassword<'_, OS::Id, OS::Password>
    ) -> ReceptionUpdatePasswordResult {
        trace_function!("Reception Update Password");

        ReceptionUpdatePasswordResult::Owner(self.owner.update_password(OwnerUpdatePassword { id, old_password, new_password }))
    }

    pub fn own(
        &mut self,
        ReceptionOwn {
            id, password, resource_id
        }: ReceptionOwn<'_, OS::Id, OS::Password, AS::ValueId>
    ) -> ReceptionOwnResult {
        trace_function!("Reception Own");

        ReceptionOwnResult::Owner(self.owner.own(OwnerOwn { id, password, resource_id }))
    }

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