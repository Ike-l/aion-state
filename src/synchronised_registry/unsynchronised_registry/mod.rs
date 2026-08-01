use std::fmt::Debug;

use stable_deref_trait::StableDeref;

use crate::prelude::{AccessStorage, Accessor, AccessorResult, AutomatedRegistry, BlacklistStorage, ControlStorage, CoordinatedReception, CredentialStorage, ManualRegistryAccessInput, ManualRegistryReplacementInput, ReceptionAllow, ReceptionCheckAccess, ReceptionDrainReservations, ReceptionGetAccess, ReceptionOwn, ReceptionRecordAccess, ReceptionRegister, ReceptionReleaseAccess, ReceptionReleaseResource, ReceptionReleaseResourceAll, ReceptionReservation, ReceptionUnallow, ReceptionUnregister, ReceptionUnreserve, ReceptionUpdatePassword, RegistryAcquireAccess, RegistryAllow, RegistryCheckAccess, RegistryContainsResource, RegistryDrainReservations, RegistryOwn, RegistryRegister, RegistryReleaseAccess, RegistryReleaseResource, RegistryReleaseResourceAll, RegistryReservation, RegistrySaferReplacement, RegistryStorage, RegistryUnallow, RegistryUnregister, RegistryUnreserve, RegistryUpdatePassword, ReservationStorage, UnsynchronisedRegistryAcquireAccessError, UnsynchronisedRegistryBlacklistAllowResult, UnsynchronisedRegistryBlacklistUnallowResult, UnsynchronisedRegistryCheckAccessResult, UnsynchronisedRegistryContainsResourceResult, UnsynchronisedRegistryDrainReservationsResult, UnsynchronisedRegistryOwnResult, UnsynchronisedRegistryRegisterResult, UnsynchronisedRegistryReleaseAccessResult, UnsynchronisedRegistryReleaseResourceAllResult, UnsynchronisedRegistryReleaseResourceResult, UnsynchronisedRegistryReservationResult, UnsynchronisedRegistrySaferReplacementResult, UnsynchronisedRegistryUnregisterResult, UnsynchronisedRegistryUnreserveResult, UnsynchronisedRegistryUpdatePasswordResult, UnsynchronisedRegistryWhitelistAllowResult, UnsynchronisedRegistryWhitelistUnallowResult, StoredValueTrait, WhitelistStorage, trace_function};

pub mod automated_registry;
pub mod coordinated_reception;
pub mod unsynchronised_registry_result;
pub mod registry_input;
#[cfg(feature = "releaser")]
pub mod impl_releaser;

#[derive(Default)]
pub struct UnsynchronisedRegistry<S, RS, AS, OS, WL, BL, CS> {
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
> UnsynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor,
{
    pub fn register(
        &self, 
        RegistryRegister {
            id, password
        }: RegistryRegister<OS::Id, OS::Password>
    ) -> UnsynchronisedRegistryRegisterResult {
        trace_function!("Unsynchronised Registry Register");

        UnsynchronisedRegistryRegisterResult::Reception(self.reception.register(ReceptionRegister { id, password }))
    }

    pub fn unregister(
        &self,
        RegistryUnregister {
            id, password
        }: &RegistryUnregister<'_, OS::Id, OS::Password>
    ) -> UnsynchronisedRegistryUnregisterResult {
        trace_function!("Unsynchronised Registry Unregister");

        UnsynchronisedRegistryUnregisterResult::Reception(self.reception.unregister(&ReceptionUnregister { id, password }))
    }

    pub fn update_password(
        &self,
        RegistryUpdatePassword {
            id, old_password, new_password
        }: RegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> UnsynchronisedRegistryUpdatePasswordResult {
        trace_function!("Unsynchronised Registry Update Password");

        UnsynchronisedRegistryUpdatePasswordResult::Reception(self.reception.update_password(ReceptionUpdatePassword { id, old_password, new_password }))
    }


    pub fn own(
        &self,
        RegistryOwn {
            id, password, resource_id
        }: RegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> UnsynchronisedRegistryOwnResult {
        trace_function!("Unsynchronised Registry Own");

        UnsynchronisedRegistryOwnResult::Reception(self.reception.own(ReceptionOwn { id, password, resource_id }))
    }

    pub fn release_resource(
        &self, 
        RegistryReleaseResource {
            id, password, resource_id
        }: &RegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> UnsynchronisedRegistryReleaseResourceResult {
        trace_function!("Unsynchronised Registry Release Resource");

        UnsynchronisedRegistryReleaseResourceResult::Reception(self.reception.release_resource(&ReceptionReleaseResource { id, password, resource_id }))
    } 

    pub fn release_resource_all<'a>(
        &self,
        RegistryReleaseResourceAll {
            id, password, 
            inputs
        }: RegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> UnsynchronisedRegistryReleaseResourceAllResult {
        trace_function!("Unsynchronised Registry Release Resource All");

        UnsynchronisedRegistryReleaseResourceAllResult::Reception(self.reception.release_resource_all(ReceptionReleaseResourceAll { id, password, inputs }))
    }


    pub fn allow_blacklist(
        &self,
        RegistryAllow {
            id, password, resource_id, access
        }: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> UnsynchronisedRegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Unsynchronised Registry Allow Blacklist");

        UnsynchronisedRegistryBlacklistAllowResult::Reception(self.reception.allow_blacklist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn allow_whitelist(
        &self,
        RegistryAllow {
            id, password, resource_id, access
        }: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> UnsynchronisedRegistryWhitelistAllowResult {
        trace_function!("Unsynchronised Registry Allow Whitelist");

        UnsynchronisedRegistryWhitelistAllowResult::Reception(self.reception.allow_whitelist(ReceptionAllow { id, password, resource_id, access }))
    }

    pub fn unallow_blacklist(
        &self,
        RegistryUnallow {
            id, password, resource_id, access
        }: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> UnsynchronisedRegistryBlacklistUnallowResult {
        trace_function!("Unsynchronised Registry Unallow Blacklist");

        UnsynchronisedRegistryBlacklistUnallowResult::Reception(self.reception.unallow_blacklist(&ReceptionUnallow { id, password, resource_id, access }))
    }

    pub fn unallow_whitelist(
        &self,
        RegistryUnallow {
            id, password, resource_id, access
        }: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> UnsynchronisedRegistryWhitelistUnallowResult {
        trace_function!("Unsynchronised Registry Unallow Whitelist");

        UnsynchronisedRegistryWhitelistUnallowResult::Reception(self.reception.unallow_whitelist(&ReceptionUnallow { id, password, resource_id, access }))
    }

    /// # Safety
    /// 
    /// No Concurrent Unique References (No writes that could modify the keys)
    pub unsafe fn check_access(
        &self,
        RegistryCheckAccess {
            user_details, resource_id, access, password
        }: &RegistryCheckAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> UnsynchronisedRegistryCheckAccessResult {
        trace_function!("Unsynchronised Registry Check Access");

        let reception_result = self.reception.check_access(&ReceptionCheckAccess { user_details: *user_details, resource_id, access, password: *password });

        if reception_result.ok() {
            return UnsynchronisedRegistryCheckAccessResult::AutomatedRegistry(unsafe { self.automated_registry.contains_key(*resource_id) })
        }

        UnsynchronisedRegistryCheckAccessResult::Reception(reception_result)
    }

    /// # Safety
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    pub unsafe fn release_access(
        &self,
        RegistryReleaseAccess {
            resource_id, access
        }: &RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> UnsynchronisedRegistryReleaseAccessResult {
        trace_function!("Unsynchronised Registry Release Access");

        UnsynchronisedRegistryReleaseAccessResult::Reception(self.reception.release_access(&ReceptionReleaseAccess { resource_id, access }))
    }

    pub fn reserve(
        &self,
        RegistryReservation {
            id, id_password, resource_id, access, password
        }: RegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> UnsynchronisedRegistryReservationResult {
        trace_function!("Unsynchronised Registry Reserve");

        UnsynchronisedRegistryReservationResult::Reception(self.reception.reserve(ReceptionReservation { id, id_password, resource_id, access, password }))
    }

    pub fn unreserve(
        &self,
        RegistryUnreserve {
            id, password, resource_id, access
        }: &RegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> UnsynchronisedRegistryUnreserveResult {
        trace_function!("Unsynchronised Registry Unreserve");

        UnsynchronisedRegistryUnreserveResult::Reception(self.reception.unreserve(&ReceptionUnreserve { id, password, resource_id, access }))
    }

    pub fn drain_reservations(
        &self,
        RegistryDrainReservations {
            id, password
        }: &RegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> UnsynchronisedRegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Unsynchronised Registry Drain Reservations");

        UnsynchronisedRegistryDrainReservationsResult::Reception(self.reception.drain_reservations(&ReceptionDrainReservations { id, password }))
    }


    /// # Safety
    /// 
    /// No Concurrent Unique References
    pub unsafe fn acquire_access<'a, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(
        &'a self,
        RegistryAcquireAccess {
            user_details, resource_id, access, password
        }: RegistryAcquireAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> Result<AccessResult, UnsynchronisedRegistryAcquireAccessError> 
        where <S as RegistryStorage>::Value: StoredValueTrait 
    {
        trace_function!("Unsynchronised Registry Acquire Access");

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
                    return Err(UnsynchronisedRegistryAcquireAccessError::AutomatedRegistry(err))
                },
            }
        }

        Err(UnsynchronisedRegistryAcquireAccessError::Reception(check_reception))
    }

    /// # Safety 
    /// 
    /// No Concurrent References
    /// 
    /// Access cannot be used to 'acquire' a reference to StableAddress itself
    /// 
    /// Insert won't invalidate concurrent access
    /// 
    /// ^ i.e do not replace a borrowed item
    pub unsafe fn safer_replace(
        &self,
        RegistrySaferReplacement {
            user_details, access, resource_id, resource, password
        }: RegistrySaferReplacement<'_, OS::Id, OS::Password, AS::Access, S::ValueId, <S::Value as StoredValueTrait>::Value, BS::Password>
    ) -> UnsynchronisedRegistrySaferReplacementResult<<S::Value as StoredValueTrait>::Value>
        where <S as RegistryStorage>::Value: StableDeref + StoredValueTrait
    {
        trace_function!("Unsynchronised Registry Safer Replace");

        let reception_result = self.reception.check_access(&ReceptionCheckAccess { user_details, resource_id: &resource_id, access, password });

        if reception_result.ok() {
            return UnsynchronisedRegistrySaferReplacementResult::AutomatedRegistry(unsafe { self.automated_registry.safer_replace(ManualRegistryReplacementInput { access, value_id: resource_id, value: resource }) })
        }

        UnsynchronisedRegistrySaferReplacementResult::Reception(reception_result)
    }

    /// # Safety
    /// 
    /// No Concurrent Unique References (No writes that could modify the keys)
    pub unsafe fn contains_resource(
        &self,
        RegistryContainsResource {
            resource_id
        }: &RegistryContainsResource<'_, S::ValueId>
    ) -> UnsynchronisedRegistryContainsResourceResult {
        trace_function!("Unsynchronised Registry Contains Resource");

        UnsynchronisedRegistryContainsResourceResult::AutomatedRegistry(unsafe { self.automated_registry.contains_key(resource_id) })
    }

    /// # Safety
    /// 
    /// No Concurrent Unique References (No writes that could modify the len)
    pub unsafe fn len(&self) -> usize {
        trace_function!("Unsynchronised Registry Len");

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
> UnsynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Clone + Accessor,
{
    pub fn get_access(
        &self,
        input: &ReceptionGetAccess<'_, AS::ValueId>
    ) -> Option<AS::Access> {
        trace_function!("Unsynchronised Registry Get Access");

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
> UnsynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        AS::ValueId: Clone
{
    /// # Safety
    /// 
    /// No Concurrent Unique References (No writes that could modify the keys)
    pub unsafe fn keys(&self) -> Vec<<S as RegistryStorage>::ValueId> {
        trace_function!("Unsynchronised Registry keys");

        unsafe { self.automated_registry.keys() }
    }
}