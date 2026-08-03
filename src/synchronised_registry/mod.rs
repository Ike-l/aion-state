use std::fmt::Debug;
#[cfg(feature = "notifier")]
use std::hash::Hash;

use stable_deref_trait::StableDeref;

use crate::prelude::{AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, ReceptionGetAccess, UnsynchronisedRegistry, RegistryAcquireAccess, SynchronisedRegistryAcquireAccessError, RegistryAllow, SynchronisedRegistryBlacklistAllowResult, SynchronisedRegistryBlacklistUnallowResult, RegistryCheckAccess, SynchronisedRegistryCheckAccessResult, RegistryContainsResource, SynchronisedRegistryContainsResourceResult, RegistryDrainReservations, SynchronisedRegistryDrainReservationsResult, RegistryOwn, SynchronisedRegistryOwnResult, RegistryRegister, SynchronisedRegistryRegisterResult, RegistryReleaseAccess, SynchronisedRegistryReleaseAccessResult, RegistryReleaseResource, RegistryReleaseResourceAll, SynchronisedRegistryReleaseResourceAllResult, SynchronisedRegistryReleaseResourceResult, RegistryReservation, SynchronisedRegistryReservationResult, RegistrySaferReplacement, SynchronisedRegistrySaferReplacementResult, RegistryStorage, RegistryUnallow, RegistryUnregister, SynchronisedRegistryUnregisterResult, RegistryUnreserve, SynchronisedRegistryUnreserveResult, RegistryUpdatePassword, SynchronisedRegistryUpdatePasswordResult, SynchronisedRegistryWhitelistAllowResult, SynchronisedRegistryWhitelistUnallowResult, ReservationStorage, StoredValueTrait, WhitelistStorage, sync::RwLock, trace_function};

pub mod unsynchronised_registry;
pub mod synchronised_registry_results;
#[cfg(feature = "releaser")]
pub mod impl_releaser;
#[cfg(feature = "notifier")]
pub mod impl_notifier;
#[cfg(feature = "async")]
pub mod impl_async;
#[cfg(all(feature = "async", feature = "releaser"))]
pub mod impl_async_releaser;
#[cfg(all(feature = "async", feature = "notifier"))]
pub mod impl_async_notifier;
#[cfg(all(feature = "notifier", feature = "releaser"))]
pub mod impl_notifier_releaser;
#[cfg(all(feature = "async", feature = "notifier", feature = "releaser"))]
pub mod impl_async_notifier_releaser;

/// Separate Sync bc the point is to not use RAII, 
/// removing the sync and making the functions take `&mut self` would require some form of RAII in mt situations
pub struct SynchronisedRegistry<S: RegistryStorage, RS, AS, OS, WS, BS, CS> {
    #[cfg(feature = "async")]
    a_sync: tokio::sync::RwLock<()>,
    #[cfg(feature = "notifier")]
    notify_queue: crate::prelude::sync::Mutex<crate::prelude::NotifyQueue<S::ValueId>>,
    sync: RwLock<()>,
    unsynchronised_registry: UnsynchronisedRegistry<S, RS, AS, OS, WS, BS, CS>,
}

impl<S, RS, AS, OS, WS, BS, CS> Default for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS>
where
    S: RegistryStorage,
    UnsynchronisedRegistry<S, RS, AS, OS, WS, BS, CS>: Default,
{
    fn default() -> Self {
        Self {
            #[cfg(feature = "async")]
            a_sync: Default::default(),
            #[cfg(feature = "notifier")]
            notify_queue: crate::prelude::sync::Mutex::new(crate::prelude::NotifyQueue::default()),
            sync: Default::default(),
            unsynchronised_registry: Default::default(),
        }
    }
}

/// # Safety
/// 
/// S::Value is Send 
/// 
/// Registry uses the `sync` lock
unsafe impl<S: RegistryStorage, RS, AS, OS, WS, BS, CS> Send for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> where S::Value: Send {}

/// # Safety
/// 
/// S::Value is Sync 
/// 
/// Registry uses the `sync` lock
unsafe impl<S: RegistryStorage, RS, AS, OS, WS, BS, CS> Sync for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> where S::Value: Sync {}

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
    pub fn register(
        &self, 
        input: RegistryRegister<OS::Id, OS::Password>
    ) -> SynchronisedRegistryRegisterResult {
        trace_function!("Synchronised Registry Register");

        let _sync = self.sync.write();

        self.unsynchronised_registry.register(input).into()
    }

    pub fn unregister(
        &self,
        input: &RegistryUnregister<'_, OS::Id, OS::Password>
    ) -> SynchronisedRegistryUnregisterResult {
        trace_function!("Synchronised Registry Unregister");

        let _sync = self.sync.write();

        self.unsynchronised_registry.unregister(input).into()
    }

    pub fn update_password(
        &self,
        input: RegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> SynchronisedRegistryUpdatePasswordResult {
        trace_function!("Synchronised Registry Update Password");

        let _sync = self.sync.write();

        self.unsynchronised_registry.update_password(input).into()
    }


    pub fn own(
        &self,
        input: RegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SynchronisedRegistryOwnResult {
        trace_function!("Synchronised Registry Own");

        let _sync = self.sync.write();

        self.unsynchronised_registry.own(input).into()
    }

    pub fn release_resource(
        &self, 
        input: &RegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SynchronisedRegistryReleaseResourceResult {
        trace_function!("Synchronised Registry Release Resource");

        let _sync = self.sync.write();

        self.unsynchronised_registry.release_resource(input).into()
    } 

    pub fn release_resource_all<'a>(
        &self,
        input: RegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> SynchronisedRegistryReleaseResourceAllResult {
        trace_function!("Synchronised Registry Release Resource All");

        let _sync = self.sync.write();

        self.unsynchronised_registry.release_resource_all(input).into()
    }


    pub fn allow_blacklist(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Synchronised Registry Allow Blacklist");

        let _sync = self.sync.write();

        self.unsynchronised_registry.allow_blacklist(input).into()
    }

    pub fn allow_whitelist(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryWhitelistAllowResult {
        trace_function!("Synchronised Registry Allow Whitelist");

        let _sync = self.sync.write();

        self.unsynchronised_registry.allow_whitelist(input).into()
    }

    pub fn unallow_blacklist(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryBlacklistUnallowResult {
        trace_function!("Synchronised Registry Unallow Blacklist");

        let _sync = self.sync.write();

        self.unsynchronised_registry.unallow_blacklist(input).into()
    }

    pub fn unallow_whitelist(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryWhitelistUnallowResult {
        trace_function!("Synchronised Registry Unallow Whitelist");

        let _sync = self.sync.write();

        self.unsynchronised_registry.unallow_whitelist(input).into()
    }

    pub fn check_access(
        &self,
        input: &RegistryCheckAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> SynchronisedRegistryCheckAccessResult {
        trace_function!("Synchronised Registry Check Access");

        let _sync = self.sync.read();

        unsafe { self.unsynchronised_registry.check_access(input).into() }
    }

    /// # Safety
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    #[cfg(feature = "notifier")]
    pub unsafe fn release_access(
        &self,
        input: &RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryReleaseAccessResult 
        where 
            S::ValueId: Eq + Hash
    {
        trace_function!("Synchronised Registry Release Access");
        
        let _sync = self.sync.write();
        self.notify_queue.lock().wake(input.resource_id);
        
        unsafe { self.unsynchronised_registry.release_access(input) }.into()
    }
    
    /// # Safety
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    #[cfg(not(feature = "notifier"))]
    pub unsafe fn release_access(
        &self,
        input: &RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryReleaseAccessResult {
        trace_function!("Synchronised Registry Release Access");

        let _sync = self.sync.write();

        unsafe { self.unsynchronised_registry.release_access(input) }.into()
    }

    pub fn reserve(
        &self,
        input: RegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> SynchronisedRegistryReservationResult {
        trace_function!("Synchronised Registry Reserve");

        let _sync = self.sync.write();

        self.unsynchronised_registry.reserve(input).into()
    }

    pub fn unreserve(
        &self,
        input: &RegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SynchronisedRegistryUnreserveResult {
        trace_function!("Synchronised Registry Unreserve");

        let _sync = self.sync.write();

        self.unsynchronised_registry.unreserve(input).into()
    }

    pub fn drain_reservations(
        &self,
        input: &RegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> SynchronisedRegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Synchronised Registry Drain Reservations");

        let _sync = self.sync.write();

        self.unsynchronised_registry.drain_reservations(input).into()
    }

    pub fn acquire_access<'a, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(
        &'a self,
        input: RegistryAcquireAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> Result<AccessResult, SynchronisedRegistryAcquireAccessError> 
        where <S as RegistryStorage>::Value: StoredValueTrait 
    {
        trace_function!("Synchronised Registry Acquire Access");

        let _sync = self.sync.write();

        unsafe { self.unsynchronised_registry.acquire_access(input).map_err(|err| err.into()) }
    }

    pub fn safer_replace(
        &self,
        input: RegistrySaferReplacement<'_, OS::Id, OS::Password, AS::Access, S::ValueId, <S::Value as StoredValueTrait>::Value, BS::Password>
    ) -> SynchronisedRegistrySaferReplacementResult<<S::Value as StoredValueTrait>::Value>
        where <S as RegistryStorage>::Value: StableDeref + StoredValueTrait
    {
        trace_function!("Synchronised Registry Safer Replace");
        
        let _sync = self.sync.write();

        unsafe { self.unsynchronised_registry.safer_replace(input).into() }
    }

    pub fn contains_resource(
        &self,
        input: &RegistryContainsResource<'_, S::ValueId>
    ) -> SynchronisedRegistryContainsResourceResult {
        trace_function!("Synchronised Registry Contains Resource");

        let _sync = self.sync.read();

        unsafe { self.unsynchronised_registry.contains_resource(input).into() }
    }

    pub fn len(&self) -> usize {
        trace_function!("Synchronised Registry Len");

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
    pub fn get_access(
        &self,
        input: &ReceptionGetAccess<'_, AS::ValueId>
    ) -> Option<AS::Access> {
        trace_function!("Synchronised Registry Get Access");

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
    pub fn keys(&self) -> Vec<<S as RegistryStorage>::ValueId> {
        trace_function!("Synchronised Registry keys");

        let _sync = self.sync.read();
        
        unsafe { self.unsynchronised_registry.keys() }
    }
}
