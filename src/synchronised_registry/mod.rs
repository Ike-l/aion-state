use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, ReleasingResult, Releaser, ReceptionGetAccess, RegistryAcquireAccess, RegistryAcquireAccessError, RegistryAllow, RegistryBlacklistAllowResult, RegistryBlacklistUnallowResult, RegistryCheckAccess, RegistryCheckAccessResult, RegistryContainsResource, RegistryContainsResourceResult, RegistryDeaccessingAcquireAccess, RegistryDeaccessingReleaseAccess, RegistryDrainReservations, RegistryDrainReservationsResult, RegistryOwn, RegistryOwnResult, RegistryRegister, RegistryRegisterResult, RegistryReleaseAccess, RegistryReleaseAccessResult, RegistryReleaseResource, RegistryReleaseResourceAll, RegistryReleaseResourceAllResult, RegistryReleaseResourceResult, RegistryReservation, RegistryReservationResult, RegistrySaferReplacement, RegistrySaferReplacementResult, RegistryStorage, RegistryUnallow, RegistryUnregister, RegistryUnregisterResult, RegistryUnreserve, RegistryUnreserveResult, RegistryUpdatePassword, RegistryUpdatePasswordResult, RegistryWhitelistAllowResult, RegistryWhitelistUnallowResult, ReservationStorage, SingularRegistry, StableAddress, WhitelistStorage, sync::{Arc, RwLock}, trace_function};

pub mod singular_registry;
pub mod registry_results;

pub mod releaser;

/// Separate Sync bc the point is to not use RAII, 
/// removing the sync and making the functions take `&mut self` would require some form of RAII in mt situations
#[derive(Default)]
pub struct SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> {
    sync: RwLock<()>,
    singular_registry: SingularRegistry<S, RS, AS, OS, WS, BS, CS>,
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
> Releaser for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor<StoredValue = S::Value> + Clone,
        AS::ValueId: Debug + Clone,
{
    type AccessResult<'a> = <AS::Access as Accessor>::AccessResult<'a> where S: 'a, RS: 'a, AS: 'a, OS: 'a, WS: 'a, BS: 'a, CS: 'a;
    type AccessError = RegistryAcquireAccessError;
    type AccessInput = RegistryDeaccessingAcquireAccess<OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>;

    type ReleaseInput = RegistryDeaccessingReleaseAccess<S::ValueId, AS::Access>;

    fn acquire_access(self: &Arc<Self>, input: Self::AccessInput) -> Result<ReleasingResult<Self::AccessResult<'_>, Self>, Self::AccessError> {
        let result = self.as_ref().acquire_access(RegistryAcquireAccess {
            user_details: input.user_details.as_ref().map(|(a, b)| { (a, b) }),
            resource_id: input.resource_id.clone(),
            access: input.access.clone(),
            password: input.password.as_ref()
        })?;

        Ok(ReleasingResult::new(result, Arc::clone(self), RegistryDeaccessingReleaseAccess {
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
        AS::Access: Debug + Accessor<StoredValue = S::Value>,
        AS::ValueId: Debug
{
    pub fn register(
        &self, 
        input: RegistryRegister<OS::Id, OS::Password>
    ) -> RegistryRegisterResult {
        trace_function!("Synchronised Registry Register");

        let _sync = self.sync.write();

        self.singular_registry.register(input).into()
    }

    pub fn unregister(
        &self,
        input: &RegistryUnregister<'_, OS::Id, OS::Password>
    ) -> RegistryUnregisterResult {
        trace_function!("Synchronised Registry Unregister");

        let _sync = self.sync.write();

        self.singular_registry.unregister(input).into()
    }

    pub fn update_password(
        &self,
        input: RegistryUpdatePassword<'_, OS::Id, OS::Password>
    ) -> RegistryUpdatePasswordResult {
        trace_function!("Synchronised Registry Update Password");

        let _sync = self.sync.write();

        self.singular_registry.update_password(input).into()
    }


    pub fn own(
        &self,
        input: RegistryOwn<'_, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryOwnResult {
        trace_function!("Synchronised Registry Own");

        let _sync = self.sync.write();

        self.singular_registry.own(input).into()
    }

    pub fn release_resource(
        &self, 
        input: &RegistryReleaseResource<'_, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryReleaseResourceResult {
        trace_function!("Synchronised Registry Release Resource");

        let _sync = self.sync.write();

        self.singular_registry.release_resource(input).into()
    } 

    pub fn release_resource_all<'a>(
        &self,
        input: RegistryReleaseResourceAll<'a, OS::Id, OS::Password, S::ValueId>
    ) -> RegistryReleaseResourceAllResult {
        trace_function!("Synchronised Registry Release Resource All");

        let _sync = self.sync.write();

        self.singular_registry.release_resource_all(input).into()
    }


    pub fn allow_blacklist(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryBlacklistAllowResult<BS::Password> {
        trace_function!("Synchronised Registry Allow Blacklist");

        let _sync = self.sync.write();

        self.singular_registry.allow_blacklist(input).into()
    }

    pub fn allow_whitelist(
        &self,
        input: RegistryAllow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryWhitelistAllowResult {
        trace_function!("Synchronised Registry Allow Whitelist");

        let _sync = self.sync.write();

        self.singular_registry.allow_whitelist(input).into()
    }

    pub fn unallow_blacklist(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryBlacklistUnallowResult {
        trace_function!("Synchronised Registry Unallow Blacklist");

        let _sync = self.sync.write();

        self.singular_registry.unallow_blacklist(input).into()
    }

    pub fn unallow_whitelist(
        &self,
        input: &RegistryUnallow<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryWhitelistUnallowResult {
        trace_function!("Synchronised Registry Unallow Whitelist");

        let _sync = self.sync.write();

        self.singular_registry.unallow_whitelist(input).into()
    }

    pub fn check_access(
        &self,
        input: &RegistryCheckAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> RegistryCheckAccessResult {
        trace_function!("Synchronised Registry Check Access");

        let _sync = self.sync.read();

        unsafe { self.singular_registry.check_access(input).into() }
    }

    /// # Safety
    /// 
    /// Resource `resource_id` corresponding with `access` MUST actually be released
    pub unsafe fn release_access(
        &self,
        input: &RegistryReleaseAccess<'_, S::ValueId, AS::Access>
    ) -> RegistryReleaseAccessResult {
        trace_function!("Synchronised Registry Release Access");

        let _sync = self.sync.write();

        unsafe { self.singular_registry.release_access(input) }.into()
    }
    // pub fn record_access() {} Done in Acquire Access


    pub fn reserve(
        &self,
        input: RegistryReservation<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> RegistryReservationResult {
        trace_function!("Synchronised Registry Reserve");

        let _sync = self.sync.write();

        self.singular_registry.reserve(input).into()
    }

    pub fn unreserve(
        &self,
        input: &RegistryUnreserve<'_, OS::Id, OS::Password, S::ValueId, AS::Access>
    ) -> RegistryUnreserveResult {
        trace_function!("Synchronised Registry Unreserve");

        let _sync = self.sync.write();

        self.singular_registry.unreserve(input).into()
    }

    pub fn drain_reservations(
        &self,
        input: &RegistryDrainReservations<'_, OS::Id, OS::Password>
    ) -> RegistryDrainReservationsResult<Vec<(S::ValueId, AS::Access)>> {
        trace_function!("Synchronised Registry Drain Reservations");

        let _sync = self.sync.write();

        self.singular_registry.drain_reservations(input).into()
    }


    pub fn acquire_access(
        &self,
        input: RegistryAcquireAccess<'_, OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>
    ) -> Result<<AS::Access as Accessor>::AccessResult<'_>, RegistryAcquireAccessError> {
        trace_function!("Synchronised Registry Acquire Access");

        let _sync = self.sync.write();

        unsafe { self.singular_registry.acquire_access(input).map_err(|err| err.into()) }
    }

    pub fn safer_replace(
        &self,
        input: RegistrySaferReplacement<'_, OS::Id, OS::Password, AS::Access, S::ValueId, <AS::Access as Accessor>::Value, BS::Password>
    ) -> RegistrySaferReplacementResult<<AS::Access as Accessor>::StoredValue>
        where
            <AS::Access as Accessor>::StoredValue: StableAddress
    {
        trace_function!("Synchronised Registry Safer Replace");
        
        let _sync = self.sync.write();

        unsafe { self.singular_registry.safer_replace(input).into() }
    }

    pub fn contains_resource(
        &self,
        input: &RegistryContainsResource<'_, S::ValueId>
    ) -> RegistryContainsResourceResult {
        trace_function!("Synchronised Registry Contains Resource");

        let _sync = self.sync.read();

        unsafe { self.singular_registry.contains_resource(input).into() }
    }

    pub fn len(&self) -> usize {
        trace_function!("Synchronised Registry Len");

        let _sync = self.sync.read();

        unsafe { self.singular_registry.len() }
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
        AS::Access: Debug + Clone + Accessor<StoredValue = S::Value>,
        AS::ValueId: Debug
{
    pub fn get_access(
        &self,
        input: &ReceptionGetAccess<'_, AS::ValueId>
    ) -> Option<AS::Access> {
        trace_function!("Synchronised Registry Get Access");

        let _sync = self.sync.read();

        self.singular_registry.get_access(input)
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
        AS::Access: Debug + Accessor<StoredValue = S::Value>,
        AS::ValueId: Debug + Clone
{
    pub fn keys(&self) -> impl Iterator<Item = <S as RegistryStorage>::ValueId> {
        trace_function!("Synchronised Registry keys");

        let _sync = self.sync.read();
        
        unsafe { self.singular_registry.keys() }
    }
}