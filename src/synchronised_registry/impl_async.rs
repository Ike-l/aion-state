use std::fmt::Debug;
#[cfg(feature = "notifier")]
use std::hash::Hash;

use stable_deref_trait::StableDeref;

use crate::prelude::{AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, ReceptionGetAccess, SynchronisedRegistry, RegistryAcquireAccess, RegistryAllow, RegistryCheckAccess, RegistryContainsResource, RegistryDrainReservations, RegistryOwn, RegistryRegister, RegistryReleaseAccess, RegistryReleaseResource, RegistryReleaseResourceAll, RegistryReservation, RegistrySaferReplacement, RegistryStorage, RegistryUnallow, RegistryUnregister, RegistryUnreserve, RegistryUpdatePassword, ReservationStorage, StoredValueTrait, SynchronisedRegistryAcquireAccessError, SynchronisedRegistryBlacklistAllowResult, SynchronisedRegistryBlacklistUnallowResult, SynchronisedRegistryCheckAccessResult, SynchronisedRegistryContainsResourceResult, SynchronisedRegistryDrainReservationsResult, SynchronisedRegistryOwnResult, SynchronisedRegistryRegisterResult, SynchronisedRegistryReleaseAccessResult, SynchronisedRegistryReleaseResourceAllResult, SynchronisedRegistryReleaseResourceResult, SynchronisedRegistryReservationResult, SynchronisedRegistrySaferReplacementResult, SynchronisedRegistryUnregisterResult, SynchronisedRegistryUnreserveResult, SynchronisedRegistryUpdatePasswordResult, SynchronisedRegistryWhitelistAllowResult, SynchronisedRegistryWhitelistUnallowResult, WhitelistStorage, trace_function};

impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor,
{
    pub async fn register_async(
        &self, 
        input: RegistryRegister<OS::Id, OS::Password>
    ) -> SynchronisedRegistryRegisterResult {
        trace_function!("Synchronised Registry Register");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.register(input).into()
    }

    pub async fn unregister_async(
        &self,
        input: &RegistryUnregister<'_, OS::Id, OS::Password>
    ) -> SynchronisedRegistryUnregisterResult {
        trace_function!("Synchronised Registry Unregister");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.unregister(input).into()
    }

    pub async fn update_password_async(
        &self,
        input: RegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> SynchronisedRegistryUpdatePasswordResult {
        trace_function!("Synchronised Registry Update Password");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.update_password(input).into()
    }


    pub async fn own_async(
        &self,
        input: RegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SynchronisedRegistryOwnResult {
        trace_function!("Synchronised Registry Own");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.own(input).into()
    }

    pub async fn release_resource_async(
        &self, 
        input: &RegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SynchronisedRegistryReleaseResourceResult {
        trace_function!("Synchronised Registry Release Resource");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.release_resource(input).into()
    } 

    pub async fn release_resource_all_async<'a>(
        &self,
        input: RegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> SynchronisedRegistryReleaseResourceAllResult {
        trace_function!("Synchronised Registry Release Resource All");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.release_resource_all(input).into()
    }


    pub async fn allow_blacklist_async(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Synchronised Registry Allow Blacklist");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.allow_blacklist(input).into()
    }

    pub async fn allow_whitelist_async(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryWhitelistAllowResult {
        trace_function!("Synchronised Registry Allow Whitelist");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.allow_whitelist(input).into()
    }

    pub async fn unallow_blacklist_async(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryBlacklistUnallowResult {
        trace_function!("Synchronised Registry Unallow Blacklist");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.unallow_blacklist(input).into()
    }

    pub async fn unallow_whitelist_async(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryWhitelistUnallowResult {
        trace_function!("Synchronised Registry Unallow Whitelist");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.unallow_whitelist(input).into()
    }

    pub async fn check_access_async(
        &self,
        input: &RegistryCheckAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> SynchronisedRegistryCheckAccessResult {
        trace_function!("Synchronised Registry Check Access");

        let _sync = self.a_sync.read().await;
        let _a_sync = self.sync.read();

        unsafe { self.unsynchronised_registry.check_access(input).into() }
    }

    /// # Safety
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    #[cfg(feature = "notifier")]
    pub async unsafe fn release_access_async(
        &self,
        input: &RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryReleaseAccessResult 
        where 
            S::ValueId: Eq + Hash
    {
        trace_function!("Synchronised Registry Release Access");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        unsafe { self.unsynchronised_registry.release_access(input) }.into()
    }

    /// # Safety
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    #[cfg(not(feature = "notifier"))]
    pub async unsafe fn release_access_async(
        &self,
        input: &RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryReleaseAccessResult {
        trace_function!("Synchronised Registry Release Access");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        unsafe { self.unsynchronised_registry.release_access(input) }.into()
    }

    pub async fn reserve_async(
        &self,
        input: RegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> SynchronisedRegistryReservationResult {
        trace_function!("Synchronised Registry Reserve");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.reserve(input).into()
    }

    pub async fn unreserve_async(
        &self,
        input: &RegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryUnreserveResult {
        trace_function!("Synchronised Registry Unreserve");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.unreserve(input).into()
    }

    pub async fn drain_reservations_async(
        &self,
        input: &RegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> SynchronisedRegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Synchronised Registry Drain Reservations");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        self.unsynchronised_registry.drain_reservations(input).into()
    }


    pub async fn acquire_access_async<'a, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(
        &'a self,
        input: RegistryAcquireAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> Result<AccessResult, SynchronisedRegistryAcquireAccessError> 
        where <S as RegistryStorage>::Value: StoredValueTrait 
    {
        trace_function!("Synchronised a_Registry Acquire Access");

        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        unsafe { self.unsynchronised_registry.acquire_access(input).map_err(|err| err.into()) }
    }

    pub async fn safer_replace_async(
        &self,
        input: RegistrySaferReplacement<'_, OS::Id, OS::Password, AS::Access, S::ValueId, <S::Value as StoredValueTrait>::Value, BS::Password>
    ) -> SynchronisedRegistrySaferReplacementResult<<S::Value as StoredValueTrait>::Value>
        where <S as RegistryStorage>::Value: StableDeref + StoredValueTrait
    {
        trace_function!("Synchronised a_Registry Safer Replace");
        
        let _a_sync = self.a_sync.write().await;
        let _sync = self.sync.write();

        unsafe { self.unsynchronised_registry.safer_replace(input).into() }
    }

    pub async fn contains_resource_async(
        &self,
        input: &RegistryContainsResource<'_, S::ValueId>
    ) -> SynchronisedRegistryContainsResourceResult {
        trace_function!("Synchronised Registry Contains Resource");

        let _a_sync = self.a_sync.read().await;
        let _sync = self.sync.read();

        unsafe { self.unsynchronised_registry.contains_resource(input).into() }
    }

    pub async fn len_async(&self) -> usize {
        trace_function!("Synchronised Registry Len");

        let _a_sync = self.a_sync.read().await;
        let _sync = self.sync.read();

        unsafe { self.unsynchronised_registry.len() }
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
> SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Clone + Accessor,
{
    pub async fn get_access_async(
        &self,
        input: &ReceptionGetAccess<'_, AS::ValueId>
    ) -> Option<AS::Access> {
        trace_function!("Synchronised Registry Get Access");

        let _a_sync = self.a_sync.read().await;
        let _sync = self.sync.read();

        self.unsynchronised_registry.get_access(input)
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
> SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor,
        AS::ValueId: Clone
{
    pub async fn keys_async(&self) -> Vec<<S as RegistryStorage>::ValueId> {
        trace_function!("Synchronised Registry keys");

        let _a_sync = self.a_sync.read().await;
        let _sync = self.sync.read();
        
        unsafe { self.unsynchronised_registry.keys() }
    }
}
