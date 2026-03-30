use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, AutomatedRegistry, BlacklistStorage, ControlStorage, CoordinatedReception, CredentialStorage, ManualRegistryAccessInput, ManualRegistryRelease, ManualRegistryReplacementInput, ReceptionAllow, ReceptionCheckAccess, ReceptionDrainReservations, ReceptionOwn, ReceptionRecordAccess, ReceptionRegister, ReceptionReleaseAccess, ReceptionReleaseResource, ReceptionReleaseResourceAll, ReceptionReservation, ReceptionUnallow, ReceptionUnregister, ReceptionUnreserve, ReceptionUpdatePassword, RegistryStorage, ReservationStorage, RegistryAcquireAccess, RegistryAcquireAccessResult, RegistryAllow, RegistryBlacklistAllowResult, RegistryBlacklistUnallowResult, RegistryCheckAccess, RegistryCheckAccessResult, RegistryContainsResource, RegistryContainsResourceResult, RegistryDrainReservations, RegistryDrainReservationsResult, RegistryOwn, RegistryOwnResult, RegistryRegister, RegistryRegisterResult, RegistryReleaseAccess, RegistryReleaseAccessResult, RegistryReleaseResource, RegistryReleaseResourceAll, RegistryReleaseResourceAllResult, RegistryReleaseResourceResult, RegistryReservation, RegistryReservationResult, RegistrySaferReplacement, RegistrySaferReplacementResult, RegistryUnallow, RegistryUnregister, RegistryUnregisterResult, RegistryUnreserve, RegistryUnreserveResult, RegistryUpdatePassword, RegistryUpdatePasswordResult, RegistryWhitelistAllowResult, RegistryWhitelistUnallowResult, StableAddress, WhitelistStorage, trace_function};

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
        RegistryRegister {
            id, password
        }: RegistryRegister<OS::Id, OS::Password>
    ) -> RegistryRegisterResult {
        trace_function!("Singular Registry Register");

        RegistryRegisterResult::Reception(self.reception.register(ReceptionRegister { id, password }))
    }

    pub fn unregister(
        &self,
        RegistryUnregister {
            id, password
        }: &RegistryUnregister<'_, OS::Id, OS::Password>
    ) -> RegistryUnregisterResult {
        trace_function!("Singular Registry Unregister");

        RegistryUnregisterResult::Reception(self.reception.unregister(&ReceptionUnregister { id, password }))
    }

    pub fn update_password(
        &self,
        RegistryUpdatePassword {
            id, old_password, new_password
        }: RegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> RegistryUpdatePasswordResult {
        trace_function!("Singular Registry Update Password");

        RegistryUpdatePasswordResult::Reception(self.reception.update_password(ReceptionUpdatePassword { id, old_password, new_password }))
    }


    pub fn own(
        &self,
        RegistryOwn {
            id, password, resource_id
        }: RegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryOwnResult {
        trace_function!("Singular Registry Own");

        RegistryOwnResult::Reception(self.reception.own(ReceptionOwn { id, password, resource_id }))
    }

    pub fn release_resource(
        &self, 
        RegistryReleaseResource {
            id, password, resource_id
        }: &RegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryReleaseResourceResult {
        trace_function!("Singular Registry Release Resource");

        RegistryReleaseResourceResult::Reception(self.reception.release_resource(&ReceptionReleaseResource { id, password, resource_id }))
    } 

    pub fn release_resource_all<'a>(
        &self,
        RegistryReleaseResourceAll {
            id, password, 
            inputs
        }: RegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryReleaseResourceAllResult {
        trace_function!("Singular Registry Release Resource All");

        RegistryReleaseResourceAllResult::Reception(self.reception.release_resource_all(ReceptionReleaseResourceAll { id, password, inputs }))
    }


    pub fn allow_blacklist(
        &self,
        RegistryAllow {
            id, password, resource_id, access
        }: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Singular Registry Allow Blacklist");

        RegistryBlacklistAllowResult::Reception(self.reception.allow_blacklist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn allow_whitelist(
        &self,
        RegistryAllow {
            id, password, resource_id, access
        }: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryWhitelistAllowResult {
        trace_function!("Singular Registry Allow Whitelist");

        RegistryWhitelistAllowResult::Reception(self.reception.allow_whitelist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn unallow_blacklist(
        &self,
        RegistryUnallow {
            id, password, resource_id, access
        }: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryBlacklistUnallowResult {
        trace_function!("Singular Registry Unallow Blacklist");

        RegistryBlacklistUnallowResult::Reception(self.reception.unallow_blacklist(&ReceptionUnallow { id, password, resource_id, access }))
    }

    pub fn unallow_whitelist(
        &self,
        RegistryUnallow {
            id, password, resource_id, access
        }: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryWhitelistUnallowResult {
        trace_function!("Singular Registry Unallow Whitelist");

        RegistryWhitelistUnallowResult::Reception(self.reception.unallow_whitelist(&ReceptionUnallow { id, password, resource_id, access }))
    }

    pub fn check_access(
        &self,
        RegistryCheckAccess {
            id, resource_id, access, password
        }: &RegistryCheckAccess<'_, OS::Id, S::ValueId, AS::Access, BS::Password>
    ) -> RegistryCheckAccessResult {
        trace_function!("Singular Registry Check Access");

        let reception_result = self.reception.check_access(&ReceptionCheckAccess { id: *id, resource_id, access, password: *password });

        if reception_result.ok() {
            return RegistryCheckAccessResult::AutomatedRegistry(self.automated_registry.contains_key(*resource_id))
        }

        RegistryCheckAccessResult::Reception(reception_result)
    }

    /// Safety:
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    pub unsafe fn release_access(
        &self,
        RegistryReleaseAccess {
            resource_id, access
        }: RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> RegistryReleaseAccessResult {
        trace_function!("Singular Registry Release Access");

        let registry_result = self.automated_registry.release(&ManualRegistryRelease { value_id: resource_id, access });
        
        if registry_result.ok() {
            return RegistryReleaseAccessResult::Reception(self.reception.release_access(&ReceptionReleaseAccess { resource_id, access }))
        }

        RegistryReleaseAccessResult::AutomatedRegistry(registry_result)
    }
    // pub fn record_access() {} Done in Acquire Access


    pub fn reserve(
        &self,
        RegistryReservation {
            id, password, resource_id, access
        }: RegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryReservationResult {
        trace_function!("Singular Registry Reserve");

        RegistryReservationResult::Reception(self.reception.reserve(ReceptionReservation { id, password, resource_id, access }))
    }

    pub fn unreserve(
        &self,
        RegistryUnreserve {
            id, password, resource_id, access
        }: &RegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryUnreserveResult {
        trace_function!("Singular Registry Unreserve");

        RegistryUnreserveResult::Reception(self.reception.unreserve(&ReceptionUnreserve { id, password, resource_id, access }))
    }

    pub fn drain_reservations(
        &self,
        RegistryDrainReservations {
            id, password
        }: &RegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> RegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Singular Registry Drain Reservations");

        RegistryDrainReservationsResult::Reception(self.reception.drain_reservations(&ReceptionDrainReservations { id, password }))
    }


    pub fn acquire_access(
        &self,
        RegistryAcquireAccess {
            id, resource_id, access, password
        }: RegistryAcquireAccess<'_, OS::Id, S::ValueId, AS::Access, BS::Password>
    ) -> RegistryAcquireAccessResult<<AS::Access as Accessor>::AccessResult<'_>> {
        trace_function!("Singular Registry Acquire Access");

        let check_reception = self.reception.check_access(&ReceptionCheckAccess { id, resource_id: &resource_id, access: &access, password });

        if check_reception.ok() {
            let registry_result = unsafe { self.automated_registry.acquire_access(ManualRegistryAccessInput { value_id: &resource_id, access: &access }) };
    
            if registry_result.ok() {
                let reception_result = self.reception.record_access(ReceptionRecordAccess { id, resource_id, access, password });

                assert!(reception_result.ok())
            }
    
            return RegistryAcquireAccessResult::AutomatedRegistry(registry_result)
        }

        RegistryAcquireAccessResult::Reception(check_reception)
    }

    pub unsafe fn safer_replace(
        &self,
        RegistrySaferReplacement {
            access, resource_id, resource
        }: RegistrySaferReplacement<'_, AS::Access, S::ValueId, <AS::Access as Accessor>::Value>
    ) -> RegistrySaferReplacementResult<<AS::Access as Accessor>::StoredValue>
        where
            <AS::Access as Accessor>::StoredValue: StableAddress
    {
        trace_function!("Singular Registry Safer Replace");

        RegistrySaferReplacementResult::AutomatedRegistry(unsafe { self.automated_registry.safer_replace(ManualRegistryReplacementInput { access, value_id: resource_id, value: resource }) })
    }

    pub fn contains_resource(
        &self,
        RegistryContainsResource {
            resource_id
        }: &RegistryContainsResource<'_, S::ValueId>
    ) -> RegistryContainsResourceResult {
        trace_function!("Singular Registry Contains Resource");

        RegistryContainsResourceResult::AutomatedRegistry(self.automated_registry.contains_key(resource_id))
    }
}