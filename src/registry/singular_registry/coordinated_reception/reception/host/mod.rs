use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accesses, Accessor, HostAccessPermissionInput, HostAccessPermissionResult, HostRecordAccessInput, HostRecordAccessResult, PermitsAccessInput, RecordAccessInput, RemoveAccessInput, RemoveAccessResult, ReservationStorage, Reservations, ReservationsAccessPermissionInput, ReservationsReserveResult, ReservationsUnreserveResult, ReserveInput, UnreserveInput, trace_function};

pub mod reservations;
pub mod accesses;

pub mod host_input;
pub mod host_result;

/// Gates accesses using reservations
/// (`reservation` semantics are applied, then `accesses` semantics are applied)
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
        AS::Access: Debug + Accessor
{
    /// Permits access if there are no conflicting reservations
    /// 
    /// And then applies `Accessor` semantics to the incoming access
    pub fn permits_access(
        &self,
        HostAccessPermissionInput {
            reserver_id, access_id, access
        }: HostAccessPermissionInput<'_, RS::ReserverId, AS::ValueId, AS::Access>
    ) -> HostAccessPermissionResult {
        trace_function!("Host Permits Access");

        let reservations_permission = self.reservations.permits_access(ReservationsAccessPermissionInput { reserver_id, access_id, access });
        if reservations_permission.ok() {
            HostAccessPermissionResult::Accesses(self.accesses.permits_access(PermitsAccessInput { access_id, access }))
        } else {
            HostAccessPermissionResult::ReservationConflict
        }
    }

    /// Automatically unreserves the access then records it
    pub fn record_access(
        &mut self,
        HostRecordAccessInput {
            reserver_id, access_id, access
        }: HostRecordAccessInput<RS::ReserverId, AS::ValueId, AS::Access>
    ) -> HostRecordAccessResult {
        trace_function!("Host Recording Access");

        let unreserve_result = if let Some(reserver_id) = reserver_id {
            Some(self.unreserve(UnreserveInput { reserver_id, access_id: &access_id, access: &access }))
        } else { None };

        let record_access_result = self.accesses.record_access(RecordAccessInput { access_id, access });

        HostRecordAccessResult {
            unreserve_result, record_access_result
        }
    }

    /// Simply a `pass through` function
    /// 
    /// Reservation semantics do not apply to releasing a current access
    pub fn release_access(
        &mut self,
        remove_access_input: RemoveAccessInput<'_, AS::ValueId, AS::Access>
    ) -> RemoveAccessResult {
        trace_function!("Host Releasing Access");

        self.accesses.release_access(remove_access_input)
    }

    /// Simply a `pass through` function
    /// 
    /// `Accesses` semantics do not apply to `reservations`
    pub fn unreserve(
        &mut self,
        unreserve_input: UnreserveInput<'_, RS::ReserverId, AS::ValueId, AS::Access>
    ) -> ReservationsUnreserveResult {
        trace_function!("Host Unreserving");

        self.reservations.unreserve(unreserve_input)
    }

    /// Simply a `pass through` function
    /// 
    /// `Accesses` semantics do not apply to `reservations`
    pub fn reserve(
        &mut self,
        reserve_input: ReserveInput<RS::ReserverId, AS::ValueId, AS::Access>
    ) -> ReservationsReserveResult {
        trace_function!("Host Reserving");

        self.reservations.reserve(reserve_input)
    }
}