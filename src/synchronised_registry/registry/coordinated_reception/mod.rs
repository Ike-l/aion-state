use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, Reception, ReceptionAllow, ReceptionBlacklistAllowResult, ReceptionBlacklistUnallowResult, ReceptionCheckAccess, ReceptionCheckAccessResult, ReceptionDrainReservations, ReceptionDrainReservationsResult, ReceptionGetAccess, ReceptionOwn, ReceptionOwnResult, ReceptionRecordAccess, ReceptionRecordAccessResult, ReceptionRegister, ReceptionRegisterResult, ReceptionReleaseAccess, ReceptionReleaseAccessResult, ReceptionReleaseResource, ReceptionReleaseResourceAll, ReceptionReleaseResourceAllResult, ReceptionReleaseResourceResult, ReceptionReservation, ReceptionReservationResult, ReceptionUnallow, ReceptionUnregister, ReceptionUnregisterResult, ReceptionUnreserve, ReceptionUnreserveResult, ReceptionUpdatePassword, ReceptionUpdatePasswordResult, ReceptionWhitelistAllowResult, ReceptionWhitelistUnallowResult, ReservationStorage, WhitelistStorage, sync::RwLock, trace_function};

pub mod reception;

#[derive(Default)]
pub struct CoordinatedReception<RS, AS, OS, PS, LS, OSS> {
    reception: RwLock<Reception<RS, AS, OS, PS, LS, OSS>>
}

impl<
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<ResourceId = BS::Id, Id = OS::Id>
> CoordinatedReception<RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor,
        AS::ValueId: Debug
{
    pub fn register(
        &self,
        input: ReceptionRegister<OS::Id, OS::Password>
    ) -> ReceptionRegisterResult {
        trace_function!("Coordinated Reception Register");
        
        self.reception.write().register(input)
    }
    
    pub fn unregister(
        &self,
        input: &ReceptionUnregister<'_, OS::Id, OS::Password>
    ) -> ReceptionUnregisterResult {
        trace_function!("Coordinated Reception Unregister");

        self.reception.write().unregister(input)
    }
    
    pub fn update_password(
        &self,
        input: ReceptionUpdatePassword<'_, OS::Id, OS::Password>
    ) -> ReceptionUpdatePasswordResult {
        trace_function!("Coordinated Reception Update Password");
        
        self.reception.write().update_password(input)
    }

    pub fn own(
        &self,
        input: ReceptionOwn<'_, OS::Id, OS::Password, AS::ValueId>
    ) -> ReceptionOwnResult {
        trace_function!("Coordinated Reception Own");

        self.reception.write().own(input)
    }

    pub fn release_resource(
        &self,
        input: &ReceptionReleaseResource<'_, OS::Id, OS::Password, AS::ValueId>
    ) -> ReceptionReleaseResourceResult {
        trace_function!("Coordinated Reception Release Resource");

        self.reception.write().release_resource(input)
    }

    pub fn release_resource_all<'a>(
        &self,
        input: ReceptionReleaseResourceAll<'a, OS::Id, OS::Password, AS::ValueId>
    ) -> ReceptionReleaseResourceAllResult {
        trace_function!("Coordinated Reception Release Resource All");

        self.reception.write().release_resource_all(input)
    }

    pub fn allow_whitelist(
        &self,
        input: ReceptionAllow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionWhitelistAllowResult {
        trace_function!("Coordinated Reception Allow Whitelist");

        self.reception.write().allow_whitelist(input)
    }

    pub fn allow_blacklist(
        &self,
        input: ReceptionAllow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionBlacklistAllowResult<BS::Password> {
        trace_function!("Coordinated Reception Allow Blacklist");

        self.reception.write().allow_blacklist(input)
    }

    pub fn unallow_whitelist(
        &self,
        input: &ReceptionUnallow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionWhitelistUnallowResult {
        trace_function!("Coordinated Reception Unallow Whitelist");

        self.reception.write().unallow_whitelist(input)
    }

    pub fn unallow_blacklist(
        &self,
        input: &ReceptionUnallow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionBlacklistUnallowResult {
        trace_function!("Coordinated Reception Unallow Blacklist");

        self.reception.write().unallow_blacklist(input)
    }

    pub fn check_access(
        &self,
        input: &ReceptionCheckAccess<'_, OS::Id, OS::Password, AS::ValueId, AS::Access, BS::Password>
    ) -> ReceptionCheckAccessResult {
        trace_function!("Coordinated Reception Check Access");

        self.reception.write().check_access(input)
    }

    pub fn release_access(
        &self,
        input: &ReceptionReleaseAccess<'_, AS::ValueId, AS::Access>
    ) -> ReceptionReleaseAccessResult {
        trace_function!("Coordinated Reception Release Access");

        self.reception.write().release_access(input)
    }

    pub fn record_access(
        &self,
        input: ReceptionRecordAccess<'_, OS::Id, OS::Password, AS::ValueId, AS::Access, BS::Password>
    ) -> ReceptionRecordAccessResult {
        trace_function!("Coordinated Reception Record Access");

        self.reception.write().record_access(input)
    }

    pub fn reserve(
        &self,
        input: ReceptionReservation<'_, OS::Id, OS::Password, AS::ValueId, AS::Access, BS::Password>
    ) -> ReceptionReservationResult {
        trace_function!("Coordinated Reception Reserve");

        self.reception.write().reserve(input)
    }
    
    pub fn unreserve(
        &self,
        input: &ReceptionUnreserve<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionUnreserveResult {
        trace_function!("Coordinated Reception Unreserve");

        self.reception.write().unreserve(input)
    }

    pub fn drain_reservations(
        &self,
        input: &ReceptionDrainReservations<'_, OS::Id, OS::Password>
    ) -> ReceptionDrainReservationsResult<Vec<(AS::ValueId, AS::Access)>> {
        trace_function!("Coordinated Reception Drain Reservations");

        self.reception.write().drain_reservations(input)
    }
}

impl<
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<ResourceId = BS::Id, Id = OS::Id>
> CoordinatedReception<RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Clone + Accessor,
        AS::ValueId: Debug
{
    pub fn get_access(
        &self,
        input: &ReceptionGetAccess<'_, AS::ValueId>
    ) -> Option<AS::Access> {
        trace_function!("Coordinated Reception Get Access");

        self.reception.write().get_access(input).cloned()
    }
}