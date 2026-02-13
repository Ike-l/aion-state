use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accesses, Accessor, HostAccessPermissionInput, HostAccessPermissionResult, HostRecordAccessInput, HostRecordAccessResult, PermitsAccessInput, RecordAccessInput, RemoveAccessInput, RemoveAccessResult, ReservationStorage, Reservations, ReservationsAccessPermissionInput, ReservationsReserveResult, ReservationsUnreserveResult, ReserveInput, UnreserveInput, trace_function};

pub mod reservations;
pub mod accesses;

pub mod host_input;
pub mod host_result;

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
    pub fn permits_access(
        &self,
        HostAccessPermissionInput {
            reserver_id, access_id, access
        }: HostAccessPermissionInput<'_, RS::ReserverId, AS::AccessId, AS::Access>
    ) -> HostAccessPermissionResult {
        trace_function!("Host Permits Access");

        let reservations_permission = self.reservations.permits_access(ReservationsAccessPermissionInput { reserver_id, access_id, access });
        if reservations_permission.ok() {
            HostAccessPermissionResult::Accesses(self.accesses.permits_access(PermitsAccessInput { access_id, access }))
        } else {
            HostAccessPermissionResult::ReservationConflict
        }
    }

    pub fn record_access(
        &mut self,
        HostRecordAccessInput {
            reserver_id, access_id, access
        }: HostRecordAccessInput<RS::ReserverId, AS::AccessId, AS::Access>
    ) -> HostRecordAccessResult {
        trace_function!("Host Record Access");

        let unreserve_result = if let Some(reserver_id) = reserver_id {
            Some(self.unreserve(UnreserveInput { reserver_id, access_id: &access_id, access: &access }))
        } else { None };

        let record_access_result = self.accesses.record_access(RecordAccessInput { access_id, access });

        HostRecordAccessResult {
            unreserve_result, record_access_result
        }
    }

    pub fn release_access(
        &mut self,
        remove_access_input: RemoveAccessInput<'_, AS::AccessId, AS::Access>
    ) -> RemoveAccessResult {
        trace_function!("Host Remove Access");

        self.accesses.release_access(remove_access_input)
    }

    pub fn unreserve(
        &mut self,
        unreserve_input: UnreserveInput<'_, RS::ReserverId, AS::AccessId, AS::Access>
    ) -> ReservationsUnreserveResult {
        trace_function!("Host Unreserve");

        self.reservations.unreserve(unreserve_input)
    }

    pub fn reserve(
        &mut self,
        reserve_input: ReserveInput<RS::ReserverId, AS::AccessId, AS::Access>
    ) -> ReservationsReserveResult {
        trace_function!("Host Reserve");

        self.reservations.reserve(reserve_input)
    }
}