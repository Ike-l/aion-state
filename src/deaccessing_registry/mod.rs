use std::fmt::Debug;

use crate::prelude::{sync::Arc, AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, ReceptionGetAccess, RegistryAcquireAccess, RegistryAcquireAccessError, RegistryAllow, RegistryBlacklistAllowResult, RegistryBlacklistUnallowResult, RegistryCheckAccess, RegistryCheckAccessResult, RegistryContainsResource, RegistryContainsResourceResult, RegistryDrainReservations, RegistryDrainReservationsResult, RegistryOwn, RegistryOwnResult, RegistryRegister, RegistryRegisterResult, RegistryReleaseResource, RegistryReleaseResourceAll, RegistryReleaseResourceAllResult, RegistryReleaseResourceResult, RegistryReservation, RegistryReservationResult, RegistrySaferReplacement, RegistrySaferReplacementResult, RegistryStorage, RegistryUnallow, RegistryUnregister, RegistryUnregisterResult, RegistryUnreserve, RegistryUnreserveResult, RegistryUpdatePassword, RegistryUpdatePasswordResult, RegistryWhitelistAllowResult, RegistryWhitelistUnallowResult, ReservationStorage, StableAddress, SynchronisedRegistry, WhitelistStorage, trace_function};

pub mod deaccessing_result;

#[derive(Default)]
pub struct DeaccessingRegistry<S, RS, AS, OS, WS, BS, CS> {
    synchronised_registry: SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS>,
}

/// # Safety
/// 
/// S::Value is Send 
/// 
/// Registry uses the `sync` lock
unsafe impl<S: RegistryStorage, RS, AS, OS, WS, BS, CS> Send for DeaccessingRegistry<S, RS, AS, OS, WS, BS, CS> where S::Value: Send {}

/// # Safety
/// 
/// S::Value is Sync 
/// 
/// Registry uses the `sync` lock
unsafe impl<S: RegistryStorage, RS, AS, OS, WS, BS, CS> Sync for DeaccessingRegistry<S, RS, AS, OS, WS, BS, CS> where S::Value: Sync {}

impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> DeaccessingRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor<StoredValue = S::Value>,
        AS::ValueId: Debug
{
    pub fn register(
        &self, 
        input: RegistryRegister<OS::Id, OS::Password>
    ) -> RegistryRegisterResult {
        trace_function!("Deaccessing Registry Register");

        self.synchronised_registry.register(input)
    }

    pub fn unregister(
        &self,
        input: &RegistryUnregister<'_, OS::Id, OS::Password>
    ) -> RegistryUnregisterResult {
        trace_function!("Deaccessing Registry Unregister");

        self.synchronised_registry.unregister(input)
    }

    pub fn update_password(
        &self,
        input: RegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> RegistryUpdatePasswordResult {
        trace_function!("Deaccessing Registry Update Password");

        self.synchronised_registry.update_password(input)
    }


    pub fn own(
        &self,
        input: RegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryOwnResult {
        trace_function!("Deaccessing Registry Own");

        self.synchronised_registry.own(input)
    }

    pub fn release_resource(
        &self, 
        input: &RegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryReleaseResourceResult {
        trace_function!("Deaccessing Registry Release Resource");

        self.synchronised_registry.release_resource(input)
    } 

    pub fn release_resource_all<'a>(
        &self,
        input: RegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryReleaseResourceAllResult {
        trace_function!("Deaccessing Registry Release Resource All");

        self.synchronised_registry.release_resource_all(input)
    }


    pub fn allow_blacklist(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Deaccessing Registry Allow Blacklist");

        self.synchronised_registry.allow_blacklist(input)
    }

    pub fn allow_whitelist(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryWhitelistAllowResult {
        trace_function!("Deaccessing Registry Allow Whitelist");

        self.synchronised_registry.allow_whitelist(input)
    }

    pub fn unallow_blacklist(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryBlacklistUnallowResult {
        trace_function!("Deaccessing Registry Unallow Blacklist");

        self.synchronised_registry.unallow_blacklist(input)
    }

    pub fn unallow_whitelist(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryWhitelistUnallowResult {
        trace_function!("Deaccessing Registry Unallow Whitelist");

        self.synchronised_registry.unallow_whitelist(input)
    }

    pub fn check_access(
        &self,
        input: &RegistryCheckAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> RegistryCheckAccessResult {
        trace_function!("Deaccessing Registry Check Access");

        self.synchronised_registry.check_access(input)
    }

    // /// # Safety
    // /// 
    // /// Resource `resource_id` corresponding with `access` MUST actually be released
    // pub unsafe fn release_access(
    //     &self,
    //     input: &RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    // ) -> RegistryReleaseAccessResult {
    //     trace_function!("Deaccessing Registry Release Access");

    //     unsafe { self.synchronised_registry.release_access(input) }
    // }

    pub fn reserve(
        &self,
        input: RegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> RegistryReservationResult {
        trace_function!("Deaccessing Registry Reserve");

        self.synchronised_registry.reserve(input)
    }

    pub fn unreserve(
        &self,
        input: &RegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryUnreserveResult {
        trace_function!("Deaccessing Registry Unreserve");

        self.synchronised_registry.unreserve(input)
    }

    pub fn drain_reservations(
        &self,
        input: &RegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> RegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Deaccessing Registry Drain Reservations");

        self.synchronised_registry.drain_reservations(input)
    }


    pub fn acquire_access(
        self: &Arc<Self>,
        input: RegistryAcquireAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> Result<<AS::Access as Accessor>::AccessResult<'_>, RegistryAcquireAccessError> {
        trace_function!("Deaccessing Registry Acquire Access");

        self.synchronised_registry.acquire_access(input)
    }

    pub fn safer_replace(
        &self,
        input: RegistrySaferReplacement<'_, OS::Id, OS::Password, AS::Access, S::ValueId, <AS::Access as Accessor>::Value, BS::Password>
    ) -> RegistrySaferReplacementResult<<AS::Access as Accessor>::StoredValue>
        where
            <AS::Access as Accessor>::StoredValue: StableAddress
    {
        trace_function!("Deaccessing Registry Safer Replace");

        self.synchronised_registry.safer_replace(input)
    }

    pub fn contains_resource(
        &self,
        input: &RegistryContainsResource<'_, S::ValueId>
    ) -> RegistryContainsResourceResult {
        trace_function!("Deaccessing Registry Contains Resource");

        self.synchronised_registry.contains_resource(input)
    }

    pub fn len(&self) -> usize {
        trace_function!("Deaccessing Registry Len");

        self.synchronised_registry.len()
    }
}

impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> DeaccessingRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Clone + Accessor<StoredValue = S::Value>,
        AS::ValueId: Debug
{
    pub fn get_access(
        &self,
        input: &ReceptionGetAccess<'_, AS::ValueId>
    ) -> Option<AS::Access> {
        trace_function!("Deaccessing Registry Get Access");

        self.synchronised_registry.get_access(input)
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
> DeaccessingRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor<StoredValue = S::Value>,
        AS::ValueId: Debug + Clone
{
    pub fn keys(&self) -> impl Iterator<Item = <S as RegistryStorage>::ValueId> {
        trace_function!("Deaccessing Registry keys");

        self.synchronised_registry.keys()
    }
}
