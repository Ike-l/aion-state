use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, Host, Owner, OwnerAllow, OwnerOwn, OwnerRegister, OwnerReleaseResource, OwnerUnregister, OwnerUpdatePassword, ReceptionAllow, ReceptionOwn, ReceptionOwnResult, ReceptionRegister, ReceptionRegisterResult, ReceptionReleaseResource, ReceptionReleaseResourceResult, ReceptionUnregister, ReceptionUnregisterResult, ReceptionUpdatePassword, ReceptionUpdatePasswordResult, ReceptionWhitelistAllowResult, ReservationStorage, WhitelistStorage, trace_function};

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
    // for: Simpler and more predictable (you know your reservations will be dropped)
    // ^-> Make drop_reservations function
    // against: Means you can deauthenticate and pass "ownership" of reservations to someone else
    pub fn unregister(
        &mut self,
        ReceptionUnregister {
            id, password
        }: &ReceptionUnregister<'_, OS::Id, OS::Password>
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

    // does this need to check if a reservation is made?
    // since there can be reservations via ReserverId with an Unregistered Owner Id,
    // if someone:
    // registers: IdA
    // makes reservations: ReserverA, ResourceA
    // unregisters
    // someone else registers: IdB
    // claims ownership over the resource: IdB, ResourceA
    // there would be a reservation conflict: ReserverA & ResourceA
    // maybe this is intentional?
    // maybe when "owning" a resource is converts all reservations to update the ReserverId to the new 

    // requires: Need to be registered (owner.authenticate().ok()) to make reservations
    // When Owning; if successful: Convert all reservations over `resource_id` to `id`
    // since can only make reservations if registered, if new ownership is successful- means they unregistered
    // so when "owning"- semantically they also claim ownership over reservations itself
    // NO, because multiple different users can hold reservations.
    // The unregisterer will need to provide or drop their reservations otherwise it "locks" the resource for an id (could just use access control)

    pub fn own(
        &mut self,
        ReceptionOwn {
            id, password, resource_id
        }: ReceptionOwn<'_, OS::Id, OS::Password, AS::ValueId>
    ) -> ReceptionOwnResult {
        trace_function!("Reception Own");

        ReceptionOwnResult::Owner(self.owner.own(OwnerOwn { id, password, resource_id }))
    }

    pub fn release_resource(
        &mut self,
        ReceptionReleaseResource {
            id, password, resource_id
        }: &ReceptionReleaseResource<'_, OS::Id, OS::Password, AS::ValueId>
    ) -> ReceptionReleaseResourceResult {
        trace_function!("Reception Release Resource");

        ReceptionReleaseResourceResult::Owner(self.owner.release_resource(&OwnerReleaseResource { id, password, resource_id }))
    }

    pub fn release_resource_all() {}

    pub fn allow_whitelist(
        &mut self,
        ReceptionAllow {
            id, password, resource_id, access
        }: ReceptionAllow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionWhitelistAllowResult {
        trace_function!("Reception Allow Whitelist");

        ReceptionWhitelistAllowResult::Owner(self.owner.allow_whitelist(OwnerAllow { id, password, resource_id, access }))
    }

    pub fn allow_blacklist() {}
    pub fn unallow_whitelist() {}
    pub fn unallow_blacklist() {}

    // Host Specific 
    pub fn check_access() {}
    pub fn release_access() {}
    pub fn record_access() {}

    pub fn reserve() {}
    pub fn unreserve() {}
    pub fn drop_reservations() {}
}