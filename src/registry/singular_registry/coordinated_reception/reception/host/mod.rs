use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accesses, AccessesCheckAccess, AccessesRecordAccess, AccessesRelease, Accessor, HostCheckAccess, HostCheckAccessResult, HostDrainReservations, HostDrainReservationsResult, HostRecordAccess, HostRecordAccessResult, HostReleaseAccess, HostReleaseAccessResult, HostReservation, HostReservationResult, HostUnreserve, HostUnreserveResult, ReservationStorage, Reservations, ReservationsCheckAccess, ReservationsDrainReservations, ReservationsReservation, ReservationsUnreserve, trace_function};

pub mod reservations;
pub mod accesses;

pub mod host_input;
pub mod host_result;

/// Gates accesses using reservations
/// (`reservation` semantics are applied, then `accesses` semantics are applied)
#[derive(Default)]
pub struct Host<RS, AS> {
    reservations: Reservations<RS>,
    accesses: Accesses<AS>,
}

impl<
    RS: ReservationStorage<AccessStorage = AS>, 
    AS: AccessStorage + Default
> Host<RS, AS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor,
        AS::ValueId: Debug
{
    /// Permits access if there are no conflicting reservations
    /// 
    /// And then applies `Accessor` semantics to the incoming access
    pub fn check_access(
        &self,
        HostCheckAccess {
            reserver_id, access_id, access
        }: &HostCheckAccess<'_, RS::ReserverId, AS::ValueId, AS::Access>
    ) -> HostCheckAccessResult {
        trace_function!("Host Permits Access");

        let reservations_permission = self.reservations.check_access(&ReservationsCheckAccess { reserver_id: *reserver_id, access_id, access });
        if reservations_permission.ok() {
            HostCheckAccessResult::Accesses(self.accesses.check_access(&AccessesCheckAccess { access_id, access }))
        } else {
            HostCheckAccessResult::ReservationConflict
        }
    }

    /// Automatically unreserves the access then records it
    pub fn record_access(
        &mut self,
        HostRecordAccess {
            reserver_id, access_id, access
        }: HostRecordAccess<RS::ReserverId, AS::ValueId, AS::Access>
    ) -> HostRecordAccessResult {
        trace_function!("Host Recording Access");

        let unreserve_result = if let Some(reserver_id) = reserver_id {
            Some(self.unreserve(&HostUnreserve { reserver_id, access_id: &access_id, access: &access }))
        } else { None };

        let record_access_result = self.accesses.record_access(AccessesRecordAccess { access_id, access });

        HostRecordAccessResult {
            unreserve_result, record_access_result
        }
    }

    /// Simply a `pass through` function
    /// 
    /// Reservation semantics do not apply to releasing a current access
    pub fn release_access(
        &mut self,
        HostReleaseAccess {
            access_id, access
        }: &HostReleaseAccess<AS::ValueId, AS::Access>
    ) -> HostReleaseAccessResult {
        trace_function!("Host Releasing Access");

        HostReleaseAccessResult::Accesses(self.accesses.release(&AccessesRelease { access_id, access }))
    }

    /// Simply a `pass through` function
    /// 
    /// `Accesses` semantics do not apply to `reservations`
    pub fn unreserve(
        &mut self,
        HostUnreserve {
            reserver_id, access_id, access
        }: &HostUnreserve<'_, RS::ReserverId, AS::ValueId, AS::Access>
    ) -> HostUnreserveResult {
        trace_function!("Host Unreserving");

        HostUnreserveResult::Reservations(self.reservations.unreserve(&ReservationsUnreserve { reserver_id, access_id, access }))
    }

    pub fn reserve(
        &mut self,
        HostReservation {
            reserver_id, access_id, access
        }: HostReservation<RS::ReserverId, AS::ValueId, AS::Access>
    ) -> HostReservationResult {
        trace_function!("Host Reserving");

        let accesses_result = self.accesses.check_access(&AccessesCheckAccess { access_id: &access_id, access: &access });

        if !accesses_result.err() {
            return HostReservationResult::Reservations(self.reservations.reserve(ReservationsReservation { reserver_id, access_id, access }))
        }

        HostReservationResult::AccessConflict(accesses_result)
    }

    pub fn drain_reservations(
        &mut self,
        HostDrainReservations {
            reserver_id
        }: &HostDrainReservations<'_, RS::ReserverId>
    ) -> HostDrainReservationsResult<Vec<(AS::ValueId, AS::Access)>> {
        trace_function!("Host Drain Reservations");

        HostDrainReservationsResult::Reservations(self.reservations.drain_reservations(&ReservationsDrainReservations { reserver_id }))
    }
}