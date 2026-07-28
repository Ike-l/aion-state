use std::fmt::Debug;

use stable_deref_trait::StableDeref;

use crate::prelude::{AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, ReceptionGetAccess, UnsynchronisedRegistry, RegistryAcquireAccess, SynchronisedRegistryAcquireAccessError, RegistryAllow, SynchronisedRegistryBlacklistAllowResult, SynchronisedRegistryBlacklistUnallowResult, RegistryCheckAccess, SynchronisedRegistryCheckAccessResult, RegistryContainsResource, SynchronisedRegistryContainsResourceResult, RegistryDrainReservations, SynchronisedRegistryDrainReservationsResult, RegistryOwn, SynchronisedRegistryOwnResult, RegistryRegister, SynchronisedRegistryRegisterResult, RegistryReleaseAccess, SynchronisedRegistryReleaseAccessResult, RegistryReleaseResource, RegistryReleaseResourceAll, SynchronisedRegistryReleaseResourceAllResult, SynchronisedRegistryReleaseResourceResult, RegistryReleasingAcquireAccess, RegistryReleasingReleaseAccess, RegistryReservation, SynchronisedRegistryReservationResult, RegistrySaferReplacement, SynchronisedRegistrySaferReplacementResult, RegistryStorage, RegistryUnallow, RegistryUnregister, SynchronisedRegistryUnregisterResult, RegistryUnreserve, SynchronisedRegistryUnreserveResult, RegistryUpdatePassword, SynchronisedRegistryUpdatePasswordResult, SynchronisedRegistryWhitelistAllowResult, SynchronisedRegistryWhitelistUnallowResult, Releaser, ReleasingResult, ReservationStorage, StoredValueTrait, WhitelistStorage, sync::RwLock, trace_function};

pub mod unsynchronised_registry;
pub mod synchronised_registry_results;

/// Separate Sync bc the point is to not use RAII, 
/// removing the sync and making the functions take `&mut self` would require some form of RAII in mt situations
#[derive(Default)]
pub struct SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> {
    sync: RwLock<()>,
    unsynchronised_registry: UnsynchronisedRegistry<S, RS, AS, OS, WS, BS, CS>,
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
> Releaser<<S::Value as StoredValueTrait>::Value> for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor + Clone,
        AS::ValueId: Clone,
        S::Value: StoredValueTrait
{
    type AccessError = SynchronisedRegistryAcquireAccessError;
    type AccessInput = RegistryReleasingAcquireAccess<OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>;

    type ReleaseInput = RegistryReleasingReleaseAccess<S::ValueId, AS::Access>;

    // because the import is from prelude
    #[allow(clippy::disallowed_types)]
    fn acquire_access<'a, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(self: &'a crate::prelude::sync::Arc<Self>, input: Self::AccessInput) -> Result<ReleasingResult<<S::Value as StoredValueTrait>::Value, AccessResult, Self>, Self::AccessError> {
        let result = self.as_ref().acquire_access(RegistryAcquireAccess {
            user_details: input.user_details.as_ref().map(|(a, b)| { (a, b) }),
            resource_id: input.resource_id.clone(),
            access: input.access.clone(),
            password: input.password.as_ref()
        })?;

        // because the import is from prelude
        #[allow(clippy::disallowed_types)]
        Ok(ReleasingResult::new(result, crate::prelude::sync::Arc::clone(self), RegistryReleasingReleaseAccess {
            resource_id: input.resource_id,
            access: input.access
        }))
    }

    fn release_access(&self, input: &Self::ReleaseInput) {
        unsafe { self.release_access(&RegistryReleaseAccess {
            resource_id: &input.resource_id,
            access: &input.access
        }) };
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
