use std::fmt::Debug;

use stable_deref_trait::StableDeref;

use crate::prelude::{AccessStorage, Accessor, AccessorResult, AutomatedRegistry, BlacklistStorage, ControlStorage, CoordinatedReception, CredentialStorage, ManualRegistryAccessInput, ManualRegistryReplacementInput, ReceptionAllow, ReceptionCheckAccess, ReceptionDrainReservations, ReceptionGetAccess, ReceptionOwn, ReceptionRecordAccess, ReceptionRegister, ReceptionReleaseAccess, ReceptionReleaseResource, ReceptionReleaseResourceAll, ReceptionReservation, ReceptionUnallow, ReceptionUnregister, ReceptionUnreserve, ReceptionUpdatePassword, RegistryAcquireAccess, RegistryAllow, RegistryCheckAccess, RegistryContainsResource, RegistryDrainReservations, RegistryOwn, RegistryRegister, RegistryReleaseAccess, RegistryReleaseResource, RegistryReleaseResourceAll, RegistryReservation, RegistrySaferReplacement, RegistryStorage, RegistryUnallow, RegistryUnregister, RegistryUnreserve, RegistryUpdatePassword, ReservationStorage, SingularRegistryAcquireAccessError, SingularRegistryBlacklistAllowResult, SingularRegistryBlacklistUnallowResult, SingularRegistryCheckAccessResult, SingularRegistryContainsResourceResult, SingularRegistryDrainReservationsResult, SingularRegistryOwnResult, SingularRegistryRegisterResult, SingularRegistryReleaseAccessResult, SingularRegistryReleaseResourceAllResult, SingularRegistryReleaseResourceResult, SingularRegistryReservationResult, SingularRegistrySaferReplacementResult, SingularRegistryUnregisterResult, SingularRegistryUnreserveResult, SingularRegistryUpdatePasswordResult, SingularRegistryWhitelistAllowResult, SingularRegistryWhitelistUnallowResult, StoredValueTrait, WhitelistStorage, trace_function};

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
        AS::Access: Debug + Accessor,
        AS::ValueId: Debug
{
    pub fn register(
        &self, 
        RegistryRegister {
            id, password
        }: RegistryRegister<OS::Id, OS::Password>
    ) -> SingularRegistryRegisterResult {
        trace_function!("Singular Registry Register");

        SingularRegistryRegisterResult::Reception(self.reception.register(ReceptionRegister { id, password }))
    }

    pub fn unregister(
        &self,
        RegistryUnregister {
            id, password
        }: &RegistryUnregister<'_, OS::Id, OS::Password>
    ) -> SingularRegistryUnregisterResult {
        trace_function!("Singular Registry Unregister");

        SingularRegistryUnregisterResult::Reception(self.reception.unregister(&ReceptionUnregister { id, password }))
    }

    pub fn update_password(
        &self,
        RegistryUpdatePassword {
            id, old_password, new_password
        }: RegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> SingularRegistryUpdatePasswordResult {
        trace_function!("Singular Registry Update Password");

        SingularRegistryUpdatePasswordResult::Reception(self.reception.update_password(ReceptionUpdatePassword { id, old_password, new_password }))
    }


    pub fn own(
        &self,
        RegistryOwn {
            id, password, resource_id
        }: RegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryOwnResult {
        trace_function!("Singular Registry Own");

        SingularRegistryOwnResult::Reception(self.reception.own(ReceptionOwn { id, password, resource_id }))
    }

    pub fn release_resource(
        &self, 
        RegistryReleaseResource {
            id, password, resource_id
        }: &RegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryReleaseResourceResult {
        trace_function!("Singular Registry Release Resource");

        SingularRegistryReleaseResourceResult::Reception(self.reception.release_resource(&ReceptionReleaseResource { id, password, resource_id }))
    } 

    pub fn release_resource_all<'a>(
        &self,
        RegistryReleaseResourceAll {
            id, password, 
            inputs
        }: RegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryReleaseResourceAllResult {
        trace_function!("Singular Registry Release Resource All");

        SingularRegistryReleaseResourceAllResult::Reception(self.reception.release_resource_all(ReceptionReleaseResourceAll { id, password, inputs }))
    }


    pub fn allow_blacklist(
        &self,
        RegistryAllow {
            id, password, resource_id, access
        }: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Singular Registry Allow Blacklist");

        SingularRegistryBlacklistAllowResult::Reception(self.reception.allow_blacklist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn allow_whitelist(
        &self,
        RegistryAllow {
            id, password, resource_id, access
        }: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryWhitelistAllowResult {
        trace_function!("Singular Registry Allow Whitelist");

        SingularRegistryWhitelistAllowResult::Reception(self.reception.allow_whitelist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn unallow_blacklist(
        &self,
        RegistryUnallow {
            id, password, resource_id, access
        }: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryBlacklistUnallowResult {
        trace_function!("Singular Registry Unallow Blacklist");

        SingularRegistryBlacklistUnallowResult::Reception(self.reception.unallow_blacklist(&ReceptionUnallow { id, password, resource_id, access }))
    }

    pub fn unallow_whitelist(
        &self,
        RegistryUnallow {
            id, password, resource_id, access
        }: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryWhitelistUnallowResult {
        trace_function!("Singular Registry Unallow Whitelist");

        SingularRegistryWhitelistUnallowResult::Reception(self.reception.unallow_whitelist(&ReceptionUnallow { id, password, resource_id, access }))
    }

    pub unsafe fn check_access(
        &self,
        RegistryCheckAccess {
            user_details, resource_id, access, password
        }: &RegistryCheckAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> SingularRegistryCheckAccessResult {
        trace_function!("Singular Registry Check Access");

        let reception_result = self.reception.check_access(&ReceptionCheckAccess { user_details: *user_details, resource_id, access, password: *password });

        if reception_result.ok() {
            return SingularRegistryCheckAccessResult::AutomatedRegistry(unsafe { self.automated_registry.contains_key(*resource_id) })
        }

        SingularRegistryCheckAccessResult::Reception(reception_result)
    }

    /// # Safety
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    pub unsafe fn release_access(
        &self,
        RegistryReleaseAccess {
            resource_id, access
        }: &RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> SingularRegistryReleaseAccessResult {
        trace_function!("Singular Registry Release Access");

        SingularRegistryReleaseAccessResult::Reception(self.reception.release_access(&ReceptionReleaseAccess { resource_id, access }))
    }

    pub fn reserve(
        &self,
        RegistryReservation {
            id, id_password, resource_id, access, password
        }: RegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> SingularRegistryReservationResult {
        trace_function!("Singular Registry Reserve");

        SingularRegistryReservationResult::Reception(self.reception.reserve(ReceptionReservation { id, id_password, resource_id, access, password }))
    }

    pub fn unreserve(
        &self,
        RegistryUnreserve {
            id, password, resource_id, access
        }: &RegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryUnreserveResult {
        trace_function!("Singular Registry Unreserve");

        SingularRegistryUnreserveResult::Reception(self.reception.unreserve(&ReceptionUnreserve { id, password, resource_id, access }))
    }

    pub fn drain_reservations(
        &self,
        RegistryDrainReservations {
            id, password
        }: &RegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> SingularRegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Singular Registry Drain Reservations");

        SingularRegistryDrainReservationsResult::Reception(self.reception.drain_reservations(&ReceptionDrainReservations { id, password }))
    }


    pub unsafe fn acquire_access<'a, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(
        &'a self,
        RegistryAcquireAccess {
            user_details, resource_id, access, password
        }: RegistryAcquireAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> Result<AccessResult, SingularRegistryAcquireAccessError> 
        where <S as RegistryStorage>::Value: StoredValueTrait 
    {
        trace_function!("Singular Registry Acquire Access");

        let check_reception = self.reception.check_access(&ReceptionCheckAccess { user_details, resource_id: &resource_id, access: &access, password });

        if check_reception.ok() {
            let registry_result = unsafe { self.automated_registry.acquire_access(ManualRegistryAccessInput { value_id: &resource_id, access: &access }) };

            match registry_result {
                Ok(accesses_result) => {
                    let reception_result = self.reception.record_access(ReceptionRecordAccess { user_details, resource_id, access, password });
        
                    assert!(reception_result.ok());
        
                    return Ok(accesses_result);
                },
                Err(err) => {
                    return Err(SingularRegistryAcquireAccessError::AutomatedRegistry(err))
                },
            }
        }

        Err(SingularRegistryAcquireAccessError::Reception(check_reception))
    }

    pub unsafe fn safer_replace(
        &self,
        RegistrySaferReplacement {
            user_details, access, resource_id, resource, password
        }: RegistrySaferReplacement<'_, OS::Id, OS::Password, AS::Access, S::ValueId, <S::Value as StoredValueTrait>::Value, BS::Password>
    ) -> SingularRegistrySaferReplacementResult<<S::Value as StoredValueTrait>::Value>
        where <S as RegistryStorage>::Value: StableDeref + StoredValueTrait
    {
        trace_function!("Singular Registry Safer Replace");

        let reception_result = self.reception.check_access(&ReceptionCheckAccess { user_details, resource_id: &resource_id, access, password });

        if reception_result.ok() {
            return SingularRegistrySaferReplacementResult::AutomatedRegistry(unsafe { self.automated_registry.safer_replace(ManualRegistryReplacementInput { access, value_id: resource_id, value: resource }) })
        }

        SingularRegistrySaferReplacementResult::Reception(reception_result)
    }

    pub unsafe fn contains_resource(
        &self,
        RegistryContainsResource {
            resource_id
        }: &RegistryContainsResource<'_, S::ValueId>
    ) -> SingularRegistryContainsResourceResult {
        trace_function!("Singular Registry Contains Resource");

        SingularRegistryContainsResourceResult::AutomatedRegistry(unsafe { self.automated_registry.contains_key(resource_id) })
    }

    pub unsafe fn len(&self) -> usize {
        trace_function!("Singular Registry Len");

        unsafe { self.automated_registry.len() }
    }
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
        AS::Access: Debug + Clone + Accessor,
        AS::ValueId: Debug
{
    pub fn get_access(
        &self,
        input: &ReceptionGetAccess<'_, AS::ValueId>
    ) -> Option<AS::Access> {
        trace_function!("Singular Registry Get Access");

        self.reception.get_access(input)
    }
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
        AS::Access: Debug + Accessor,
        AS::ValueId: Debug + Clone
{
    pub unsafe fn keys(&self) -> impl Iterator<Item = <S as RegistryStorage>::ValueId> {
        trace_function!("Singular Registry keys");

        unsafe { self.automated_registry.keys() }
    }
}