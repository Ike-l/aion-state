use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, AutomatedRegistry, BlacklistStorage, ControlStorage, CoordinatedReception, CredentialStorage, ManualRegistryCheckAccess, ReceptionAllow, ReceptionCheckAccess, ReceptionOwn, ReceptionRegister, ReceptionReleaseResource, ReceptionReleaseResourceAll, ReceptionUnallow, ReceptionUnregister, ReceptionUpdatePassword, RegistryStorage, ReservationStorage, SingularRegistryAllow, SingularRegistryBlacklistAllowResult, SingularRegistryBlacklistUnallowResult, SingularRegistryCheckAccess, SingularRegistryCheckAccessResult, SingularRegistryOwn, SingularRegistryOwnResult, SingularRegistryRegister, SingularRegistryRegisterResult, SingularRegistryReleaseResource, SingularRegistryReleaseResourceAll, SingularRegistryReleaseResourceAllResult, SingularRegistryReleaseResourceResult, SingularRegistryUnallow, SingularRegistryUnregister, SingularRegistryUnregisterResult, SingularRegistryUpdatePassword, SingularRegistryUpdatePasswordResult, SingularRegistryWhitelistAllowResult, SingularRegistryWhitelistUnallowResult, WhitelistStorage, trace_function};

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
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<ResourceId = BS::Id, Id = OS::Id>
> SingularRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor<StoredValue = S::Value>,
        AS::ValueId: Debug
{
    pub fn register(
        &mut self, 
        SingularRegistryRegister {
            id, password
        }: SingularRegistryRegister<OS::Id, OS::Password>
    ) -> SingularRegistryRegisterResult {
        trace_function!("Singular Registry Register");

        SingularRegistryRegisterResult::Reception(self.reception.register(ReceptionRegister { id, password }))
    }

    pub fn unregister(
        &mut self,
        SingularRegistryUnregister {
            id, password
        }: &SingularRegistryUnregister<'_, OS::Id, OS::Password>
    ) -> SingularRegistryUnregisterResult {
        trace_function!("Singular Registry Unregister");

        SingularRegistryUnregisterResult::Reception(self.reception.unregister(&ReceptionUnregister { id, password }))
    }

    pub fn update_password(
        &mut self,
        SingularRegistryUpdatePassword {
            id, old_password, new_password
        }: SingularRegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> SingularRegistryUpdatePasswordResult {
        trace_function!("Singular Registry Update Password");

        SingularRegistryUpdatePasswordResult::Reception(self.reception.update_password(ReceptionUpdatePassword { id, old_password, new_password }))
    }


    pub fn own(
        &mut self,
        SingularRegistryOwn {
            id, password, resource_id
        }: SingularRegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryOwnResult {
        trace_function!("Singular Registry Own");

        SingularRegistryOwnResult::Reception(self.reception.own(ReceptionOwn { id, password, resource_id }))
    }

    pub fn release_resource(
        &mut self, 
        SingularRegistryReleaseResource {
            id, password, resource_id
        }: &SingularRegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryReleaseResourceResult {
        trace_function!("Singular Registry Release Resource");

        SingularRegistryReleaseResourceResult::Reception(self.reception.release_resource(&ReceptionReleaseResource { id, password, resource_id }))
    } 

    pub fn release_resource_all<'a>(
        &mut self,
        SingularRegistryReleaseResourceAll {
            id, password, 
            inputs
        }: SingularRegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryReleaseResourceAllResult<'a, OS::Id, AS::ValueId> {
        trace_function!("Singular Registry Release Resource All");

        SingularRegistryReleaseResourceAllResult::Reception(self.reception.release_resource_all(ReceptionReleaseResourceAll { id, password, inputs }))
    }


    pub fn allow_blacklist(
        &mut self,
        SingularRegistryAllow {
            id, password, resource_id, access
        }: SingularRegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Singular Registry Allow Blacklist");

        SingularRegistryBlacklistAllowResult::Reception(self.reception.allow_blacklist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn allow_whitelist(
        &mut self,
        SingularRegistryAllow {
            id, password, resource_id, access
        }: SingularRegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryWhitelistAllowResult {
        trace_function!("Singular Registry Allow Whitelist");

        SingularRegistryWhitelistAllowResult::Reception(self.reception.allow_whitelist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn unallow_blacklist(
        &mut self,
        SingularRegistryUnallow {
            id, password, resource_id, access
        }: &SingularRegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryBlacklistUnallowResult {
        trace_function!("Singular Registry Unallow Blacklist");

        SingularRegistryBlacklistUnallowResult::Reception(self.reception.unallow_blacklist(&ReceptionUnallow { id, password, resource_id, access }))
    }

    pub fn unallow_whitelist(
        &mut self,
        SingularRegistryUnallow {
            id, password, resource_id, access
        }: &SingularRegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryWhitelistUnallowResult {
        trace_function!("Singular Registry Unallow Whitelist");

        SingularRegistryWhitelistUnallowResult::Reception(self.reception.unallow_whitelist(&ReceptionUnallow { id, password, resource_id, access }))
    }

    pub fn check_access(
        &mut self,
        SingularRegistryCheckAccess {
            id, resource_id, access, password
        }: &SingularRegistryCheckAccess<'_, OS::Id, S::ValueId, AS::Access, BS::Password>
    ) -> SingularRegistryCheckAccessResult {
        trace_function!("Singular Registry Check Access");

        let reception_result = self.reception.check_access(&ReceptionCheckAccess { id: *id, resource_id, access, password: *password });

        if reception_result.ok() {
            return SingularRegistryCheckAccessResult::AutomatedRegistry(self.automated_registry.check_access(&ManualRegistryCheckAccess { value_id: *resource_id, access: *access }))
        }

        SingularRegistryCheckAccessResult::Reception(reception_result)
    }

    pub fn release_access(
        &mut self,
    
    ) {}
    // pub fn record_access() {} Done in Acquire Access

    pub fn reserve() {}
    pub fn unreserve() {}
    pub fn drain_reservations() {}

    pub fn acquire_access() {}
    pub fn safer_replace() {}
    pub fn contains_resource() {}
}