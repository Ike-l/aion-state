use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, ReceptionGetAccess, RegistryAcquireAccess, RegistryAcquireAccessError, RegistryAllow, RegistryBlacklistAllowResult, RegistryBlacklistUnallowResult, RegistryCheckAccess, RegistryCheckAccessResult, RegistryContainsResource, RegistryContainsResourceResult, RegistryDrainReservations, RegistryDrainReservationsResult, RegistryOwn, RegistryOwnResult, RegistryRegister, RegistryRegisterResult, RegistryReleaseAccess, RegistryReleaseAccessResult, RegistryReleaseResource, RegistryReleaseResourceAll, RegistryReleaseResourceAllResult, RegistryReleaseResourceResult, RegistryReservation, RegistryReservationResult, RegistrySaferReplacement, RegistrySaferReplacementResult, RegistryStorage, RegistryUnallow, RegistryUnregister, RegistryUnregisterResult, RegistryUnreserve, RegistryUnreserveResult, RegistryUpdatePassword, RegistryUpdatePasswordResult, RegistryWhitelistAllowResult, RegistryWhitelistUnallowResult, ReservationStorage, SingularRegistry, StableAddress, WhitelistStorage, sync::RwLock, trace_function};

pub mod singular_registry;

pub mod registry_results;

/// Separate Sync bc the point is to not use RAII, 
/// removing the sync and making the functions take `&mut self` would require some form of RAII in mt situations
#[derive(Default)]
pub struct Registry<S, RS, AS, OS, WS, BS, CS> {
    sync: RwLock<()>,
    singular_registry: SingularRegistry<S, RS, AS, OS, WS, BS, CS>,
}

/// # Safety
/// 
/// S::Value is Send 
/// 
/// Registry uses the `sync` lock
unsafe impl<S: RegistryStorage, RS, AS, OS, WS, BS, CS> Send for Registry<S, RS, AS, OS, WS, BS, CS> where S::Value: Send {}

/// # Safety
/// 
/// S::Value is Sync 
/// 
/// Registry uses the `sync` lock
unsafe impl<S: RegistryStorage, RS, AS, OS, WS, BS, CS> Sync for Registry<S, RS, AS, OS, WS, BS, CS> where S::Value: Sync {}

impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> Registry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor<StoredValue = S::Value>,
        AS::ValueId: Debug
{
    pub fn register(
        &self, 
        input: RegistryRegister<OS::Id, OS::Password>
    ) -> RegistryRegisterResult {
        trace_function!("Registry Register");

        let _sync = self.sync.write();

        self.singular_registry.register(input).into()
    }

    pub fn unregister(
        &self,
        input: &RegistryUnregister<'_, OS::Id, OS::Password>
    ) -> RegistryUnregisterResult {
        trace_function!("Registry Unregister");

        let _sync = self.sync.write();

        self.singular_registry.unregister(input).into()
    }

    pub fn update_password(
        &self,
        input: RegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> RegistryUpdatePasswordResult {
        trace_function!("Registry Update Password");

        let _sync = self.sync.write();

        self.singular_registry.update_password(input).into()
    }


    pub fn own(
        &self,
        input: RegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryOwnResult {
        trace_function!("Registry Own");

        let _sync = self.sync.write();

        self.singular_registry.own(input).into()
    }

    pub fn release_resource(
        &self, 
        input: &RegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryReleaseResourceResult {
        trace_function!("Registry Release Resource");

        let _sync = self.sync.write();

        self.singular_registry.release_resource(input).into()
    } 

    pub fn release_resource_all<'a>(
        &self,
        input: RegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryReleaseResourceAllResult {
        trace_function!("Registry Release Resource All");

        let _sync = self.sync.write();

        self.singular_registry.release_resource_all(input).into()
    }


    pub fn allow_blacklist(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Registry Allow Blacklist");

        let _sync = self.sync.write();

        self.singular_registry.allow_blacklist(input).into()
    }

    pub fn allow_whitelist(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryWhitelistAllowResult {
        trace_function!("Registry Allow Whitelist");

        let _sync = self.sync.write();

        self.singular_registry.allow_whitelist(input).into()
    }

    pub fn unallow_blacklist(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryBlacklistUnallowResult {
        trace_function!("Registry Unallow Blacklist");

        let _sync = self.sync.write();

        self.singular_registry.unallow_blacklist(input).into()
    }

    pub fn unallow_whitelist(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryWhitelistUnallowResult {
        trace_function!("Registry Unallow Whitelist");

        let _sync = self.sync.write();

        self.singular_registry.unallow_whitelist(input).into()
    }

    pub fn check_access(
        &self,
        input: &RegistryCheckAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> RegistryCheckAccessResult {
        trace_function!("Registry Check Access");

        let _sync = self.sync.read();

        self.singular_registry.check_access(input).into()
    }

    /// # Safety
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    pub unsafe fn release_access(
        &self,
        input: &RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> RegistryReleaseAccessResult {
        trace_function!("Registry Release Access");

        let _sync = self.sync.write();

        unsafe { self.singular_registry.release_access(input) }.into()
    }
    // pub fn record_access() {} Done in Acquire Access


    pub fn reserve(
        &self,
        input: RegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> RegistryReservationResult {
        trace_function!("Registry Reserve");

        let _sync = self.sync.write();

        self.singular_registry.reserve(input).into()
    }

    pub fn unreserve(
        &self,
        input: &RegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryUnreserveResult {
        trace_function!("Registry Unreserve");

        let _sync = self.sync.write();

        self.singular_registry.unreserve(input).into()
    }

    pub fn drain_reservations(
        &self,
        input: &RegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> RegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Registry Drain Reservations");

        let _sync = self.sync.write();

        self.singular_registry.drain_reservations(input).into()
    }


    pub fn acquire_access(
        &self,
        input: RegistryAcquireAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> Result<<AS::Access as Accessor>::AccessResult<'_>, RegistryAcquireAccessError> {
        trace_function!("Registry Acquire Access");

        let _sync = self.sync.write();

        self.singular_registry.acquire_access(input).map_err(|err| err.into())
    }

    pub fn safer_replace(
        &self,
        input: RegistrySaferReplacement<'_, OS::Id, OS::Password, AS::Access, S::ValueId, <AS::Access as Accessor>::Value, BS::Password>
    ) -> RegistrySaferReplacementResult<<AS::Access as Accessor>::StoredValue>
        where
            <AS::Access as Accessor>::StoredValue: StableAddress
    {
        trace_function!("Registry Safer Replace");
        
        let _sync = self.sync.write();

        self.singular_registry.safer_replace(input).into()
    }

    pub fn contains_resource(
        &self,
        input: &RegistryContainsResource<'_, S::ValueId>
    ) -> RegistryContainsResourceResult {
        trace_function!("Registry Contains Resource");

        let _sync = self.sync.read();

        self.singular_registry.contains_resource(input).into()
    }

    pub fn len(&self) -> usize {
        trace_function!("Registry Len");

        let _sync = self.sync.read();

        self.singular_registry.len()
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
> Registry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Clone + Accessor<StoredValue = S::Value>,
        AS::ValueId: Debug
{
    pub fn get_access(
        &self,
        input: &ReceptionGetAccess<'_, AS::ValueId>
    ) -> Option<AS::Access> {
        trace_function!("Registry Get Access");

        let _sync = self.sync.read();

        self.singular_registry.get_access(input)
    }
}