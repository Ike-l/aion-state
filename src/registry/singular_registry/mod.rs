use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, AutomatedRegistry, BlacklistStorage, ControlStorage, CoordinatedReception, CredentialStorage, ReceptionAllow, ReceptionOwn, ReceptionRegister, ReceptionReleaseResource, ReceptionReleaseResourceAll, ReceptionUnregister, ReceptionUpdatePassword, RegistryStorage, ReservationStorage, SingularRegistryAllow, SingularRegistryBlacklistAllowResult, SingularRegistryOwn, SingularRegistryOwnResult, SingularRegistryRegister, SingularRegistryRegisterResult, SingularRegistryReleaseResource, SingularRegistryReleaseResourceAll, SingularRegistryReleaseResourceAllResult, SingularRegistryReleaseResourceResult, SingularRegistryUnregister, SingularRegistryUnregisterResult, SingularRegistryUpdatePassword, SingularRegistryUpdatePasswordResult, SingularRegistryWhitelistAllowResult, WhitelistStorage};

pub mod automated_registry;
pub mod coordinated_reception;
pub mod singular_registry_result;
pub mod singular_registry_input;

pub struct SingularRegistry<S, RS, AS, OS, WL, BL, CS> {
    automated_registry: AutomatedRegistry<S>,
    reception: CoordinatedReception<RS, AS, OS, WL, BL, CS>,
}

impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<ResourceId = BS::Id, Id = OS::Id>
> SingularRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor,
        AS::ValueId: Debug
{
    pub fn register(
        &mut self, 
        SingularRegistryRegister {
            id, password
        }: SingularRegistryRegister<OS::Id, OS::Password>
    ) -> SingularRegistryRegisterResult {
        SingularRegistryRegisterResult::Reception(self.reception.register(ReceptionRegister { id, password }))
    }

    pub fn unregister(
        &mut self,
        SingularRegistryUnregister {
            id, password
        }: &SingularRegistryUnregister<'_, OS::Id, OS::Password>
    ) -> SingularRegistryUnregisterResult {
        SingularRegistryUnregisterResult::Reception(self.reception.unregister(&ReceptionUnregister { id, password }))
    }

    pub fn update_password(
        &mut self,
        SingularRegistryUpdatePassword {
            id, old_password, new_password
        }: SingularRegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> SingularRegistryUpdatePasswordResult {
        SingularRegistryUpdatePasswordResult::Reception(self.reception.update_password(ReceptionUpdatePassword { id, old_password, new_password }))
    }


    pub fn own(
        &mut self,
        SingularRegistryOwn {
            id, password, resource_id
        }: SingularRegistryOwn<'_, OS::Id, OS::Password, AS::ValueId>
    ) -> SingularRegistryOwnResult {
        SingularRegistryOwnResult::Reception(self.reception.own(ReceptionOwn { id, password, resource_id }))
    }

    pub fn release_resource(
        &mut self, 
        SingularRegistryReleaseResource {
            id, password, resource_id
        }: &SingularRegistryReleaseResource<'_, OS::Id, OS::Password, AS::ValueId>
    ) -> SingularRegistryReleaseResourceResult {
        SingularRegistryReleaseResourceResult::Reception(self.reception.release_resource(&ReceptionReleaseResource { id, password, resource_id }))
    } 

    pub fn release_resource_all<'a>(
        &mut self,
        SingularRegistryReleaseResourceAll {
            id, password, 
            inputs
        }: SingularRegistryReleaseResourceAll<'a, OS::Id, OS::Password, AS::ValueId>
    ) -> SingularRegistryReleaseResourceAllResult<'a, OS::Id, AS::ValueId> {
        SingularRegistryReleaseResourceAllResult::Reception(self.reception.release_resource_all(ReceptionReleaseResourceAll { id, password, inputs }))
    }


    pub fn allow_blacklist(
        &mut self,
        SingularRegistryAllow {
            id, password, resource_id, access
        }: SingularRegistryAllow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> SingularRegistryBlacklistAllowResult<BS::Password> {
        SingularRegistryBlacklistAllowResult::Reception(self.reception.allow_blacklist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn allow_whitelist(
        &mut self,
        SingularRegistryAllow {
            id, password, resource_id, access
        }: SingularRegistryAllow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> SingularRegistryWhitelistAllowResult {
        SingularRegistryWhitelistAllowResult::Reception(self.reception.allow_whitelist(ReceptionAllow { id, password, resource_id, access }))
    }
    pub fn unallow_blacklist() {}
    pub fn unallow_whitelist() {}

    pub fn check_access() {}
    pub fn release_access() {}
    // pub fn record_access() {} Done in Acquire Access

    pub fn reserve() {}
    pub fn unreserve() {}
    pub fn drain_reservations() {}

    pub fn acquire_access() {}
    pub fn safer_replace() {}
    pub fn contains_resource() {}
}