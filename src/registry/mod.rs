use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, RegistryStorage, ReservationStorage, SingularRegistry, SingularRegistryAcquireAccess, SingularRegistryAcquireAccessResult, SingularRegistryAllow, SingularRegistryBlacklistAllowResult, SingularRegistryBlacklistUnallowResult, SingularRegistryCheckAccess, SingularRegistryCheckAccessResult, SingularRegistryContainsResource, SingularRegistryContainsResourceResult, SingularRegistryDrainReservations, SingularRegistryDrainReservationsResult, SingularRegistryOwn, SingularRegistryOwnResult, SingularRegistryRegister, SingularRegistryRegisterResult, SingularRegistryReleaseAccess, SingularRegistryReleaseAccessResult, SingularRegistryReleaseResource, SingularRegistryReleaseResourceAll, SingularRegistryReleaseResourceAllResult, SingularRegistryReleaseResourceResult, SingularRegistryReservation, SingularRegistryReservationResult, SingularRegistrySaferReplacement, SingularRegistrySaferReplacementResult, SingularRegistryUnallow, SingularRegistryUnregister, SingularRegistryUnregisterResult, SingularRegistryUnreserve, SingularRegistryUnreserveResult, SingularRegistryUpdatePassword, SingularRegistryUpdatePasswordResult, SingularRegistryWhitelistAllowResult, SingularRegistryWhitelistUnallowResult, StableAddress, WhitelistStorage, sync::RwLock, trace_function};

pub mod singular_registry;

/// Separate Sync bc the point is to not use RAII, 
/// removing the sync and making the functions take `&mut self` would require some form of RAII in mt situations
pub struct Registry<S, RS, AS, OS, PS, LS, OSS> {
    sync: RwLock<()>,
    singular_registry: SingularRegistry<S, RS, AS, OS, PS, LS, OSS>,
}

impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<ResourceId = BS::Id, Id = OS::Id>
> Registry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor<StoredValue = S::Value>,
        AS::ValueId: Debug
{
    pub fn register(
        &self, 
        input: SingularRegistryRegister<OS::Id, OS::Password>
    ) -> SingularRegistryRegisterResult {
        trace_function!("Registry Register");

        let _sync = self.sync.write();

        self.singular_registry.register(input)
    }

    pub fn unregister(
        &self,
        input: &SingularRegistryUnregister<'_, OS::Id, OS::Password>
    ) -> SingularRegistryUnregisterResult {
        trace_function!("Registry Unregister");

        let _sync = self.sync.write();

        self.singular_registry.unregister(input)
    }

    pub fn update_password(
        &self,
        input: SingularRegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> SingularRegistryUpdatePasswordResult {
        trace_function!("Registry Update Password");

        let _sync = self.sync.write();

        self.singular_registry.update_password(input)
    }


    pub fn own(
        &self,
        input: SingularRegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryOwnResult {
        trace_function!("Registry Own");

        let _sync = self.sync.write();

        self.singular_registry.own(input)
    }

    pub fn release_resource(
        &self, 
        input: &SingularRegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryReleaseResourceResult {
        trace_function!("Registry Release Resource");

        let _sync = self.sync.write();

        self.singular_registry.release_resource(input)
    } 

    pub fn release_resource_all<'a>(
        &self,
        input: SingularRegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> SingularRegistryReleaseResourceAllResult<'a, OS::Id, AS::ValueId> {
        trace_function!("Registry Release Resource All");

        let _sync = self.sync.write();

        self.singular_registry.release_resource_all(input)
    }


    pub fn allow_blacklist(
        &self,
        input: SingularRegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Registry Allow Blacklist");

        let _sync = self.sync.write();

        self.singular_registry.allow_blacklist(input)
    }

    pub fn allow_whitelist(
        &self,
        input: SingularRegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryWhitelistAllowResult {
        trace_function!("Registry Allow Whitelist");

        let _sync = self.sync.write();

        self.singular_registry.allow_whitelist(input)
    }

    pub fn unallow_blacklist(
        &self,
        input: &SingularRegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryBlacklistUnallowResult {
        trace_function!("Registry Unallow Blacklist");

        let _sync = self.sync.write();

        self.singular_registry.unallow_blacklist(input)
    }

    pub fn unallow_whitelist(
        &self,
        input: &SingularRegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryWhitelistUnallowResult {
        trace_function!("Registry Unallow Whitelist");

        let _sync = self.sync.write();

        self.singular_registry.unallow_whitelist(input)
    }

    pub fn check_access(
        &self,
        input: &SingularRegistryCheckAccess<'_, OS::Id, S::ValueId, AS::Access, BS::Password>
    ) -> SingularRegistryCheckAccessResult {
        trace_function!("Registry Check Access");

        let _sync = self.sync.read();

        self.singular_registry.check_access(input)
    }

    /// Safety:
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    pub unsafe fn release_access(
        &self,
        input: SingularRegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> SingularRegistryReleaseAccessResult {
        trace_function!("Registry Release Access");

        let _sync = self.sync.write();

        unsafe { self.singular_registry.release_access(input) }
    }
    // pub fn record_access() {} Done in Acquire Access


    pub fn reserve(
        &self,
        input: SingularRegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryReservationResult {
        trace_function!("Registry Reserve");

        let _sync = self.sync.write();

        self.singular_registry.reserve(input)
    }

    pub fn unreserve(
        &self,
        input: &SingularRegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> SingularRegistryUnreserveResult {
        trace_function!("Registry Unreserve");

        let _sync = self.sync.write();

        self.singular_registry.unreserve(input)
    }

    pub fn drain_reservations(
        &self,
        input: &SingularRegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> SingularRegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Registry Drain Reservations");

        let _sync = self.sync.write();

        self.singular_registry.drain_reservations(input)
    }


    pub fn acquire_access(
        &self,
        input: SingularRegistryAcquireAccess<'_, OS::Id, S::ValueId, AS::Access, BS::Password>
    ) -> SingularRegistryAcquireAccessResult<<AS::Access as Accessor>::AccessResult<'_>> {
        trace_function!("Registry Acquire Access");

        let _sync = self.sync.write();

        self.singular_registry.acquire_access(input)
    }

    pub unsafe fn safer_replace(
        &self,
        input: SingularRegistrySaferReplacement<'_, AS::Access, S::ValueId, <AS::Access as Accessor>::Value>
    ) -> SingularRegistrySaferReplacementResult<<AS::Access as Accessor>::StoredValue>
        where
            <AS::Access as Accessor>::StoredValue: StableAddress
    {
        trace_function!("Registry Safer Replace");
        
        let _sync = self.sync.write();

        unsafe { self.singular_registry.safer_replace(input) }
    }

    pub fn contains_resource(
        &self,
        input: &SingularRegistryContainsResource<'_, S::ValueId>
    ) -> SingularRegistryContainsResourceResult {
        trace_function!("Registry Contains Resource");

        let _sync = self.sync.read();

        self.singular_registry.contains_resource(input)
    }
}