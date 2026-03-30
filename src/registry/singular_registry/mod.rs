use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, AutomatedRegistry, BlacklistStorage, ControlStorage, CoordinatedReception, CredentialStorage, ManualRegistryAccessInput, ManualRegistryRelease, ManualRegistryReplacementInput, ReceptionAllow, ReceptionCheckAccess, ReceptionDrainReservations, ReceptionOwn, ReceptionRecordAccess, ReceptionRegister, ReceptionReleaseAccess, ReceptionReleaseResource, ReceptionReleaseResourceAll, ReceptionReservation, ReceptionUnallow, ReceptionUnregister, ReceptionUnreserve, ReceptionUpdatePassword, RegistryStorage, ReservationStorage, SingularRegistryAcquireAccess, SingularRegistryAcquireAccessResult, SingularRegistryAllow, SingularRegistryBlacklistAllowResult, SingularRegistryBlacklistUnallowResult, SingularRegistryCheckAccess, SingularRegistryCheckAccessResult, SingularRegistryContainsResource, SingularRegistryContainsResourceResult, SingularRegistryDrainReservations, SingularRegistryDrainReservationsResult, SingularRegistryOwn, SingularRegistryOwnResult, SingularRegistryRegister, SingularRegistryRegisterResult, SingularRegistryReleaseAccess, SingularRegistryReleaseAccessResult, SingularRegistryReleaseResource, SingularRegistryReleaseResourceAll, SingularRegistryReleaseResourceAllResult, SingularRegistryReleaseResourceResult, SingularRegistryReservation, SingularRegistryReservationResult, SingularRegistrySaferReplacement, SingularRegistrySaferReplacementResult, SingularRegistryUnallow, SingularRegistryUnregister, SingularRegistryUnregisterResult, SingularRegistryUnreserve, SingularRegistryUnreserveResult, SingularRegistryUpdatePassword, SingularRegistryUpdatePasswordResult, SingularRegistryWhitelistAllowResult, SingularRegistryWhitelistUnallowResult, StableAddress, WhitelistStorage, trace_function};

pub mod automated_registry;
pub mod coordinated_reception;
pub mod singular_registry_result;
pub mod singular_registry_input;

#[derive(Default)]
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
        &self, 
        SingularRegistryRegister {
            id, password
        }: SingularRegistryRegister<OS::Id, OS::Password>
    ) -> SingularRegistryRegisterResult {
        trace_function!("Singular Registry Register");

        SingularRegistryRegisterResult::Reception(self.reception.register(ReceptionRegister { id, password }))
    }

    pub fn unregister(
        &self,
        SingularRegistryUnregister {
            id, password
        }: &SingularRegistryUnregister<'_, OS::Id, OS::Password>
    ) -> SingularRegistryUnregisterResult {
        trace_function!("Singular Registry Unregister");

        SingularRegistryUnregisterResult::Reception(self.reception.unregister(&ReceptionUnregister { id, password }))
    }

    pub fn update_password(
        &self,
        SingularRegistryUpdatePassword {
            id, old_password, new_password
        }: SingularRegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> SingularRegistryUpdatePasswordResult {
        trace_function!("Singular Registry Update Password");

        SingularRegistryUpdatePasswordResult::Reception(self.reception.update_password(ReceptionUpdatePassword { id, old_password, new_password }))
    }


    pub fn own(
        &self,
        SingularRegistryOwn {
            id, password, resource_id
        }: SingularRegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryOwnResult {
        trace_function!("Singular Registry Own");

        SingularRegistryOwnResult::Reception(self.reception.own(ReceptionOwn { id, password, resource_id }))
    }

    pub fn release_resource(
        &self, 
        SingularRegistryReleaseResource {
            id, password, resource_id
        }: &SingularRegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryReleaseResourceResult {
        trace_function!("Singular Registry Release Resource");

        SingularRegistryReleaseResourceResult::Reception(self.reception.release_resource(&ReceptionReleaseResource { id, password, resource_id }))
    } 

    pub fn release_resource_all<'a>(
        &self,
        SingularRegistryReleaseResourceAll {
            id, password, 
            inputs
        }: SingularRegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryReleaseResourceAllResult<'a, OS::Id, AS::ValueId> {
        trace_function!("Singular Registry Release Resource All");

        SingularRegistryReleaseResourceAllResult::Reception(self.reception.release_resource_all(ReceptionReleaseResourceAll { id, password, inputs }))
    }


    pub fn allow_blacklist(
        &self,
        SingularRegistryAllow {
            id, password, resource_id, access
        }: SingularRegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Singular Registry Allow Blacklist");

        SingularRegistryBlacklistAllowResult::Reception(self.reception.allow_blacklist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn allow_whitelist(
        &self,
        SingularRegistryAllow {
            id, password, resource_id, access
        }: SingularRegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryWhitelistAllowResult {
        trace_function!("Singular Registry Allow Whitelist");

        SingularRegistryWhitelistAllowResult::Reception(self.reception.allow_whitelist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn unallow_blacklist(
        &self,
        SingularRegistryUnallow {
            id, password, resource_id, access
        }: &SingularRegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryBlacklistUnallowResult {
        trace_function!("Singular Registry Unallow Blacklist");

        SingularRegistryBlacklistUnallowResult::Reception(self.reception.unallow_blacklist(&ReceptionUnallow { id, password, resource_id, access }))
    }

    pub fn unallow_whitelist(
        &self,
        SingularRegistryUnallow {
            id, password, resource_id, access
        }: &SingularRegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryWhitelistUnallowResult {
        trace_function!("Singular Registry Unallow Whitelist");

        SingularRegistryWhitelistUnallowResult::Reception(self.reception.unallow_whitelist(&ReceptionUnallow { id, password, resource_id, access }))
    }

    pub fn check_access(
        &self,
        SingularRegistryCheckAccess {
            id, resource_id, access, password
        }: &SingularRegistryCheckAccess<'_, OS::Id, S::ValueId, AS::Access, BS::Password>
    ) -> SingularRegistryCheckAccessResult {
        trace_function!("Singular Registry Check Access");

        let reception_result = self.reception.check_access(&ReceptionCheckAccess { id: *id, resource_id, access, password: *password });

        if reception_result.ok() {
            return SingularRegistryCheckAccessResult::AutomatedRegistry(self.automated_registry.contains_key(*resource_id))
        }

        SingularRegistryCheckAccessResult::Reception(reception_result)
    }

    /// Safety:
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    pub unsafe fn release_access(
        &self,
        SingularRegistryReleaseAccess {
            resource_id, access
        }: SingularRegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> SingularRegistryReleaseAccessResult {
        trace_function!("Singular Registry Release Access");

        let registry_result = self.automated_registry.release(&ManualRegistryRelease { value_id: resource_id, access });
        
        if registry_result.ok() {
            return SingularRegistryReleaseAccessResult::Reception(self.reception.release_access(&ReceptionReleaseAccess { resource_id, access }))
        }

        SingularRegistryReleaseAccessResult::AutomatedRegistry(registry_result)
    }
    // pub fn record_access() {} Done in Acquire Access


    pub fn reserve(
        &self,
        SingularRegistryReservation {
            id, password, resource_id, access
        }: SingularRegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryReservationResult {
        trace_function!("Singular Registry Reserve");

        SingularRegistryReservationResult::Reception(self.reception.reserve(ReceptionReservation { id, password, resource_id, access }))
    }

    pub fn unreserve(
        &self,
        SingularRegistryUnreserve {
            id, password, resource_id, access
        }: &SingularRegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryUnreserveResult {
        trace_function!("Singular Registry Unreserve");

        SingularRegistryUnreserveResult::Reception(self.reception.unreserve(&ReceptionUnreserve { id, password, resource_id, access }))
    }

    pub fn drain_reservations(
        &self,
        SingularRegistryDrainReservations {
            id, password
        }: &SingularRegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> SingularRegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Singular Registry Drain Reservations");

        SingularRegistryDrainReservationsResult::Reception(self.reception.drain_reservations(&ReceptionDrainReservations { id, password }))
    }


    pub fn acquire_access(
        &self,
        SingularRegistryAcquireAccess {
            id, resource_id, access, password
        }: SingularRegistryAcquireAccess<'_, OS::Id, S::ValueId, AS::Access, BS::Password>
    ) -> SingularRegistryAcquireAccessResult<<AS::Access as Accessor>::AccessResult<'_>> {
        trace_function!("Singular Registry Acquire Access");

        let check_reception = self.reception.check_access(&ReceptionCheckAccess { id, resource_id: &resource_id, access: &access, password });

        if check_reception.ok() {
            let registry_result = unsafe { self.automated_registry.acquire_access(ManualRegistryAccessInput { value_id: &resource_id, access: &access }) };
    
            if registry_result.ok() {
                let reception_result = self.reception.record_access(ReceptionRecordAccess { id, resource_id, access, password });

                assert!(reception_result.ok())
            }
    
            return SingularRegistryAcquireAccessResult::AutomatedRegistry(registry_result)
        }

        SingularRegistryAcquireAccessResult::Reception(check_reception)
    }

    pub unsafe fn safer_replace(
        &self,
        SingularRegistrySaferReplacement {
            access, resource_id, resource
        }: SingularRegistrySaferReplacement<'_, AS::Access, S::ValueId, <AS::Access as Accessor>::Value>
    ) -> SingularRegistrySaferReplacementResult<<AS::Access as Accessor>::StoredValue>
        where
            <AS::Access as Accessor>::StoredValue: StableAddress
    {
        trace_function!("Singular Registry Safer Replace");

        SingularRegistrySaferReplacementResult::AutomatedRegistry(unsafe { self.automated_registry.safer_replace(ManualRegistryReplacementInput { access, value_id: resource_id, value: resource }) })
    }

    pub fn contains_resource(
        &self,
        SingularRegistryContainsResource {
            resource_id
        }: &SingularRegistryContainsResource<'_, S::ValueId>
    ) -> SingularRegistryContainsResourceResult {
        trace_function!("Singular Registry Contains Resource");

        SingularRegistryContainsResourceResult::AutomatedRegistry(self.automated_registry.contains_key(resource_id))
    }
}