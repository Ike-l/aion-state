use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, Reception, ReceptionAllow, ReceptionBlacklistAllowResult, ReceptionBlacklistUnallowResult, ReceptionCheckAccess, ReceptionCheckAccessResult, ReceptionDrainReservations, ReceptionDrainReservationsResult, ReceptionOwn, ReceptionOwnResult, ReceptionRecordAccess, ReceptionRecordAccessResult, ReceptionRegister, ReceptionRegisterResult, ReceptionReleaseAccess, ReceptionReleaseAccessResult, ReceptionReleaseResource, ReceptionReleaseResourceAll, ReceptionReleaseResourceAllResult, ReceptionReleaseResourceResult, ReceptionReservation, ReceptionReservationResult, ReceptionUnallow, ReceptionUnregister, ReceptionUnregisterResult, ReceptionUnreserve, ReceptionUnreserveResult, ReceptionUpdatePassword, ReceptionUpdatePasswordResult, ReceptionWhitelistAllowResult, ReceptionWhitelistUnallowResult, ReservationStorage, WhitelistStorage, sync::RwLock, trace_function};

pub mod reception;

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
        &mut self,
        input: ReceptionRegister<OS::Id, OS::Password>
    ) -> ReceptionRegisterResult {
        self.reception.write().register(input)
    }
    
    pub fn unregister(
        &mut self,
        input: &ReceptionUnregister<'_, OS::Id, OS::Password>
    ) -> ReceptionUnregisterResult {
        trace_function!("Coordinated Reception Unregister");

        self.reception.write().unregister(input)
    }
    
    pub fn update_password(
        &mut self,
        input: ReceptionUpdatePassword<'_, OS::Id, OS::Password>
    ) -> ReceptionUpdatePasswordResult {
        trace_function!("Coordinated Reception Update Password");
        self.reception.write().update_password(input)
    }

    pub fn own(
        &mut self,
        input: ReceptionOwn<'_, OS::Id, OS::Password, AS::ValueId>
    ) -> ReceptionOwnResult {
        trace_function!("Coordinated Reception Own");

        self.reception.write().own(input)
    }

    pub fn release_resource(
        &mut self,
        input: &ReceptionReleaseResource<'_, OS::Id, OS::Password, AS::ValueId>
    ) -> ReceptionReleaseResourceResult {
        trace_function!("Coordinated Reception Release Resource");

        self.reception.write().release_resource(input)
    }

    pub fn release_resource_all<'a>(
        &mut self,
        input: ReceptionReleaseResourceAll<'a, OS::Id, OS::Password, AS::ValueId>
    ) -> ReceptionReleaseResourceAllResult<'a, OS::Id, AS::ValueId> {
        trace_function!("Coordinated Reception Release Resource All");

        self.reception.write().release_resource_all(input)
    }

    pub fn allow_whitelist(
        &mut self,
        input: ReceptionAllow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionWhitelistAllowResult {
        trace_function!("Coordinated Reception Allow Whitelist");

        self.reception.write().allow_whitelist(input)
    }

    pub fn allow_blacklist(
        &mut self,
        input: ReceptionAllow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionBlacklistAllowResult<BS::Password> {
        trace_function!("Coordinated Reception Allow Blacklist");

        self.reception.write().allow_blacklist(input)
    }

    pub fn unallow_whitelist(
        &mut self,
        input: &ReceptionUnallow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionWhitelistUnallowResult {
        trace_function!("Coordinated Reception Unallow Whitelist");

        self.reception.write().unallow_whitelist(input)
    }

    pub fn unallow_blacklist(
        &mut self,
        input: &ReceptionUnallow<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionBlacklistUnallowResult {
        trace_function!("Coordinated Reception Unallow Blacklist");

        self.reception.write().unallow_blacklist(input)
    }

    pub fn check_access(
        &self,
        input: &ReceptionCheckAccess<'_, OS::Id, AS::ValueId, AS::Access, BS::Password>
    ) -> ReceptionCheckAccessResult {
        trace_function!("Coordinated Reception Check Access");

        self.reception.read().check_access(input)
    }

    pub fn release_access(
        &mut self,
        input: &ReceptionReleaseAccess<'_, AS::ValueId, AS::Access>
    ) -> ReceptionReleaseAccessResult {
        trace_function!("Coordinated Reception Release Access");

        self.reception.write().release_access(input)
    }

    pub fn record_access(
        &mut self,
        input: ReceptionRecordAccess<'_, OS::Id, AS::ValueId, AS::Access, BS::Password>
    ) -> ReceptionRecordAccessResult {
        trace_function!("Coordinated Reception Record Access");

        self.reception.write().record_access(input)
    }

    pub fn reserve(
        &mut self,
        input: ReceptionReservation<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionReservationResult {
        trace_function!("Coordinated Reception Reserve");

        self.reception.write().reserve(input)
    }
    
    pub fn unreserve(
        &mut self,
        input: &ReceptionUnreserve<'_, OS::Id, OS::Password, AS::ValueId, AS::Access>
    ) -> ReceptionUnreserveResult {
        trace_function!("Coordinated Reception Unreserve");

        self.reception.write().unreserve(input)
    }

    pub fn drain_reservations(
        &mut self,
        input: &ReceptionDrainReservations<'_, OS::Id, OS::Password>
    ) -> ReceptionDrainReservationsResult<Vec<(AS::ValueId, AS::Access)>> {
        trace_function!("Coordinated Reception Drain Reservations");

        self.reception.write().drain_reservations(input)
    }
}