use std::fmt::Debug;
#[cfg(feature = "notifier")]
use std::hash::Hash;

use stable_deref_trait::StableDeref;

use crate::prelude::{AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, ReceptionGetAccess, RegistryAcquireAccess, RegistryAllow, RegistryCheckAccess, RegistryCheckOwner, SynchronisedRegistryCheckOwnerResult, RegistryContainsResource, RegistryDrainReservations, RegistryIsOwned, RegistryOwn, RegistryRegister, RegistryReleaseAccess, RegistryReleaseResource, RegistryReleaseResourceAll, RegistryReplacement, RegistryReservation, RegistryStorage, RegistryUnallow, RegistryUnregister, RegistryUnreserve, RegistryUpdatePassword, ReservationStorage, StoredValueTrait, SynchronisedRegistry, SynchronisedRegistryAcquireAccessError, SynchronisedRegistryBlacklistAllowResult, SynchronisedRegistryBlacklistUnallowResult, SynchronisedRegistryCheckAccessResult, SynchronisedRegistryCheckedReplacementResult, SynchronisedRegistryContainsResourceResult, SynchronisedRegistryDrainReservationsResult, SynchronisedRegistryOwnResult, SynchronisedRegistryReallocatingReplacementResult, SynchronisedRegistryRegisterResult, SynchronisedRegistryReleaseAccessResult, SynchronisedRegistryReleaseResourceAllResult, SynchronisedRegistryReleaseResourceResult, SynchronisedRegistryReservationResult, SynchronisedRegistryUnregisterResult, SynchronisedRegistryUnreserveResult, SynchronisedRegistryUpdatePasswordResult, SynchronisedRegistryWhitelistAllowResult, SynchronisedRegistryWhitelistUnallowResult, WhitelistStorage, trace_function};

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
        trace_function!("Synchronised Registry Register Async");

        let _a_sync = self.a_sync.write().await;
        self.register(input)
    }

    pub async fn unregister_async(
        &self,
        input: &RegistryUnregister<'_, OS::Id, OS::Password>
    ) -> SynchronisedRegistryUnregisterResult {
        trace_function!("Synchronised Registry Unregister Async");

        let _a_sync = self.a_sync.write().await;
        self.unregister(input)
    }

    pub async fn update_password_async(
        &self,
        input: RegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> SynchronisedRegistryUpdatePasswordResult {
        trace_function!("Synchronised Registry Update Password Async");

        let _a_sync = self.a_sync.write().await;
        self.update_password(input)
    }


    pub async fn own_async(
        &self,
        input: RegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SynchronisedRegistryOwnResult {
        trace_function!("Synchronised Registry Own Async");

        let _a_sync = self.a_sync.write().await;
        self.own(input)
    }

    pub async fn is_owned_async(
        &self, 
        input: &RegistryIsOwned<'_, CS::ResourceId>
    ) -> bool {
        trace_function!("Synchronised Registry Is Owned Async");

        let _a_sync = self.a_sync.read().await;

        self.is_owned(input)
    }

    pub async fn check_owner_async(
        &self,
        input: &RegistryCheckOwner<'_, OS::Id, CS::ResourceId>
    ) -> SynchronisedRegistryCheckOwnerResult {
        trace_function!("Synchronised Registry Check Owner Async");

        let _a_sync = self.a_sync.read().await;

        self.check_owner(input)
    }

    pub async fn release_resource_async(
        &self, 
        input: &RegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SynchronisedRegistryReleaseResourceResult {
        trace_function!("Synchronised Registry Release Resource Async");

        let _a_sync = self.a_sync.write().await;
        self.release_resource(input)
    } 

    pub async fn release_resource_all_async<'a>(
        &self,
        input: RegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> SynchronisedRegistryReleaseResourceAllResult {
        trace_function!("Synchronised Registry Release Resource All Async");

        let _a_sync = self.a_sync.write().await;
        self.release_resource_all(input)
    }


    pub async fn allow_blacklist_async(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Synchronised Registry Allow Blacklist Async");

        let _a_sync = self.a_sync.write().await;
        self.allow_blacklist(input)
    }

    pub async fn allow_whitelist_async(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryWhitelistAllowResult {
        trace_function!("Synchronised Registry Allow Whitelist Async");

        let _a_sync = self.a_sync.write().await;
        self.allow_whitelist(input)
    }

    pub async fn unallow_blacklist_async(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryBlacklistUnallowResult {
        trace_function!("Synchronised Registry Unallow Blacklist Async");

        let _a_sync = self.a_sync.write().await;
        self.unallow_blacklist(input)
    }

    pub async fn unallow_whitelist_async(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryWhitelistUnallowResult {
        trace_function!("Synchronised Registry Unallow Whitelist Async");

        let _a_sync = self.a_sync.write().await;
        self.unallow_whitelist(input)
    }

    pub async fn check_access_async(
        &self,
        input: &RegistryCheckAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> SynchronisedRegistryCheckAccessResult {
        trace_function!("Synchronised Registry Check Access Async");

        let _sync = self.a_sync.read().await;
        self.check_access(input)
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
        trace_function!("Synchronised Registry Release Access Async");

        let _a_sync = self.a_sync.write().await;
        unsafe { self.release_access(input) }
    }

    /// # Safety
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    #[cfg(not(feature = "notifier"))]
    pub async unsafe fn release_access_async(
        &self,
        input: &RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryReleaseAccessResult {
        trace_function!("Synchronised Registry Release Access Async");

        let _a_sync = self.a_sync.write().await;
        unsafe { unsynchronised_registry.release_access(input) }
    }

    pub async fn reserve_async(
        &self,
        input: RegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> SynchronisedRegistryReservationResult {
        trace_function!("Synchronised Registry Reserve Async");

        let _a_sync = self.a_sync.write().await;
        self.reserve(input)
    }

    pub async fn unreserve_async(
        &self,
        input: &RegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryUnreserveResult {
        trace_function!("Synchronised Registry Unreserve Async");

        let _a_sync = self.a_sync.write().await;
        self.unreserve(input)
    }

    pub async fn drain_reservations_async(
        &self,
        input: &RegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> SynchronisedRegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Synchronised Registry Drain Reservations Async");

        let _a_sync = self.a_sync.write().await;
        self.drain_reservations(input)
    }


    pub async fn acquire_access_async<'a, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(
        &'a self,
        input: RegistryAcquireAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> Result<AccessResult, SynchronisedRegistryAcquireAccessError> 
        where <S as RegistryStorage>::Value: StoredValueTrait 
    {
        trace_function!("Synchronised Registry Acquire Access Async");

        let _a_sync = self.a_sync.write().await;
        self.acquire_access(input)
    }

    pub async fn reallocating_replace_async(
        &self,
        input: RegistryReplacement<'_, OS::Id, OS::Password, AS::Access, S::ValueId, <S::Value as StoredValueTrait>::Value, BS::Password>
    ) -> SynchronisedRegistryReallocatingReplacementResult<<S::Value as StoredValueTrait>::Value>
        where <S as RegistryStorage>::Value: StableDeref + StoredValueTrait
    {
        trace_function!("Synchronised Registry Reallocating Replace Async");
        
        let _a_sync = self.a_sync.write().await;
        self.reallocating_replace(input)
    }

    pub async fn checked_replace_async(
        &self,
        input: RegistryReplacement<'_, OS::Id, OS::Password, AS::Access, S::ValueId, <S::Value as StoredValueTrait>::Value, BS::Password>
    ) -> SynchronisedRegistryCheckedReplacementResult<<S::Value as StoredValueTrait>::Value>
        where <S as RegistryStorage>::Value: StoredValueTrait
    {
        trace_function!("Synchronised Registry Checked Replace Async");
        
        let _a_sync = self.a_sync.write().await;
        self.checked_replace(input)
    }

    pub async fn contains_resource_async(
        &self,
        input: &RegistryContainsResource<'_, S::ValueId>
    ) -> SynchronisedRegistryContainsResourceResult {
        trace_function!("Synchronised Registry Contains Resource Async");

        let _a_sync = self.a_sync.read().await;
        self.contains_resource(input)
    }

    pub async fn len_async(&self) -> usize {
        trace_function!("Synchronised Registry Len Async");

        let _a_sync = self.a_sync.read().await;
        self.len()
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
        trace_function!("Synchronised Registry Get Access Async");

        let _a_sync = self.a_sync.read().await;
        self.get_access(input)
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
        trace_function!("Synchronised Registry keys Async");

        let _a_sync = self.a_sync.read().await;
        self.keys()
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
        RS::ReserverId: Clone,
{
    pub async fn registered_async(
        &self
    ) -> Vec<OS::Id> {
        trace_function!("Synchronised Registry Registered Async");

        let _a_sync = self.a_sync.write().await;

        self.registered()
    }
}