use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, Host, HostCheckAccess, HostRecordAccess, HostReleaseAccess, HostReservation, HostUnreserve, Owner, OwnerAllow, OwnerAuthenticate, OwnerCheckAccess, OwnerOwn, OwnerRegister, OwnerReleaseResource, OwnerReleaseResourceAll, OwnerUnallow, OwnerUnregister, OwnerUpdatePassword, ReceptionAllow, ReceptionBlacklistAllowResult, ReceptionBlacklistUnallowResult, ReceptionCheckAccess, ReceptionCheckAccessResult, ReceptionOwn, ReceptionOwnResult, ReceptionRecordAccess, ReceptionRecordAccessResult, ReceptionRegister, ReceptionRegisterResult, ReceptionReleaseAccess, ReceptionReleaseAccessResult, ReceptionReleaseResource, ReceptionReleaseResourceAll, ReceptionReleaseResourceAllResult, ReceptionReleaseResourceResult, ReceptionReservation, ReceptionReservationResult, ReceptionUnallow, ReceptionUnregister, ReceptionUnregisterResult, ReceptionUnreserve, ReceptionUnreserveResult, ReceptionUpdatePassword, ReceptionUpdatePasswordResult, ReceptionWhitelistAllowResult, ReceptionWhitelistUnallowResult, ReservationStorage, WhitelistStorage, trace_function};

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

    pub fn release_resource_all<'a>(
        &mut self,
        ReceptionReleaseResourceAll {
            id, password,
            inputs
        }: ReceptionReleaseResourceAll<'a, OS::Id, OS::Password, AS::ValueId>
    ) -> ReceptionReleaseResourceAllResult<'a, OS::Id, AS::ValueId> {
        trace_function!("Reception Release Reosurce All");

        ReceptionReleaseResourceAllResult::Owner(self.owner.release_resource_all(OwnerReleaseResourceAll { id, password, inputs}))
    }

    pub fn allow_whitelist(
        &mut self,
        ReceptionAllow {
            id, password, resource_id, access
        }: ReceptionAllow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionWhitelistAllowResult {
        trace_function!("Reception Allow Whitelist");

        ReceptionWhitelistAllowResult::Owner(self.owner.allow_whitelist(OwnerAllow { id, password, resource_id, access }))
    }

    pub fn allow_blacklist(
        &mut self,
        ReceptionAllow {
            id, password, resource_id, access
        }: ReceptionAllow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionBlacklistAllowResult<BS::Password> {
        trace_function!("Reception Allow Blacklist");

        ReceptionBlacklistAllowResult::Owner(self.owner.allow_blacklist(OwnerAllow { id, password, resource_id, access }))
    }

    pub fn unallow_whitelist(
        &mut self,
        ReceptionUnallow {
            id, password, resource_id, access
        }: &ReceptionUnallow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionWhitelistUnallowResult {
        trace_function!("Reception Unallow Whitelist");

        ReceptionWhitelistUnallowResult::Owner(self.owner.unallow_whitelist(&OwnerUnallow { id, password, resource_id, access }))
    }

    pub fn unallow_blacklist(
        &mut self,
        ReceptionUnallow {
            id, password, resource_id, access
        }: &ReceptionUnallow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionBlacklistUnallowResult {
        trace_function!("Reception Unallow Blacklist");

        ReceptionBlacklistUnallowResult::Owner(self.owner.unallow_blacklist(&OwnerUnallow { id, password, resource_id, access }))
    }

    // Host Specific 
    pub fn check_access(
        &self,
        ReceptionCheckAccess {
            id, resource_id, access, password
        }: &ReceptionCheckAccess<'_, OS::Id, AS::ValueId, AS::Access, BS::Password>
    ) -> ReceptionCheckAccessResult {
        trace_function!("Reception Check Access");

        let check_owner = self.owner.check_access(&OwnerCheckAccess { id: *id, resource_id, access, password: *password });
        if check_owner.ok() {
            return ReceptionCheckAccessResult::Host(self.host.check_access(&HostCheckAccess { reserver_id: *id, access_id: *resource_id, access }))
        }

        ReceptionCheckAccessResult::Denied(check_owner)
    }

    // anyone can release any access?
    // unless i track which id gets which access
    // i suppose anyone should be able to release accesses- the parent struct can make the caller "unsafe" for implementers to ensure
    // i.e An implementor can implement `release` as a drop behaviour of a struct holding the resource
    // but since the resource is not associated with this level there is no reason to add those semantics
    pub fn release_access(
        &mut self,
        ReceptionReleaseAccess {
            resource_id, access
        }: &ReceptionReleaseAccess<'_, AS::ValueId, AS::Access>
    ) -> ReceptionReleaseAccessResult {
        trace_function!("Reception Release Access");

        ReceptionReleaseAccessResult::Host(self.host.release_access(&HostReleaseAccess { access_id: *resource_id, access }))
    }

    // authenticate?
    // when recording- check access as well?
    // 
    pub fn record_access(
        &mut self,
        ReceptionRecordAccess {
            id, resource_id, access, password
        }: ReceptionRecordAccess<'_, OS::Id, AS::ValueId, AS::Access, BS::Password>
    ) -> ReceptionRecordAccessResult {
        trace_function!("Reception Record Access");

        let check_owner = self.owner.check_access(&OwnerCheckAccess { id, resource_id: &resource_id, access: &access, password });
        if check_owner.ok() {
            return ReceptionRecordAccessResult::Host(self.host.record_access(HostRecordAccess { reserver_id: id, access_id: resource_id, access}))
        }

        ReceptionRecordAccessResult::Denied(check_owner)
    }

    pub fn reserve(
        &mut self,
        ReceptionReservation {
            id, password, resource_id, access
        }: ReceptionReservation<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionReservationResult {
        trace_function!("Reception Reserve");

        let authentication_result = self.owner.authenticate(&OwnerAuthenticate { id: &id, password  });
        if authentication_result.ok() {
            return ReceptionReservationResult::Host(self.host.reserve(HostReservation { reserver_id: id, access_id: resource_id, access }))
        }

        ReceptionReservationResult::Denied(authentication_result)
    }
    
    pub fn unreserve(
        &mut self,
        ReceptionUnreserve {
            id, password, resource_id, access
        }: ReceptionUnreserve<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionUnreserveResult {
        trace_function!("Reception Unreserve");

        let authentication_result = self.owner.authenticate(&OwnerAuthenticate { id, password });
        if authentication_result.ok() {
            return ReceptionUnreserveResult::Host(self.host.unreserve(&HostUnreserve { reserver_id: id, access_id: resource_id, access }))
        }

        ReceptionUnreserveResult::Denied(authentication_result)
    }

    pub fn drop_reservations() {}
}