use std::fmt::Debug;

use tracing::span;

use crate::prelude::{AccessStorage, Accesses, Accessor, FUNCTION_LEVEL, HostAccessPermissionInput, HostAccessPermissionResult, HostRecordAccessInput, HostRecordAccessResult, PermitsAccessInput, RecordAccessInput, RemoveAccessInput, RemoveAccessResult, ReservationStorage, Reservations, ReservationsAccessPermissionInput, UnreserveInput};

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
        RS::Key: Debug + PartialEq,
        AS::Value: Debug + Accessor
{
    pub fn permits_access(
        &self,
        HostAccessPermissionInput {
            reserver, access_key, access
        }: HostAccessPermissionInput<'_, RS::Key, AS::Key, AS::Value>
    ) -> HostAccessPermissionResult {
        let span = span!(FUNCTION_LEVEL, "Host Permits Access");
        let _enter = span.enter();

        let reservations_permission = self.reservations.permits_access(ReservationsAccessPermissionInput { reserver, access_key, access });
        if reservations_permission.ok() {
            HostAccessPermissionResult::Accesses(self.accesses.permits_access(PermitsAccessInput { access_key, access }))
        } else {
            HostAccessPermissionResult::ReservationConflict
        }
    }

    pub fn record_access(
        &mut self,
        HostRecordAccessInput {
            reserver, access_key, access
        }: HostRecordAccessInput<RS::Key, AS::Key, AS::Value>
    ) -> HostRecordAccessResult {
        let span = span!(FUNCTION_LEVEL, "Host Record Access");
        let _enter = span.enter();

        let unreserve_result = if let Some(reserver) = reserver {
            Some(self.reservations.unreserve(UnreserveInput { reserver, access_key: &access_key, access: &access }))
        } else { None };

        let record_access_result = self.accesses.record_access(RecordAccessInput { access_key, access });

        HostRecordAccessResult {
            unreserve_result, record_access_result
        }
    }

    pub fn remove_access(
        &mut self,
        remove_access_input: RemoveAccessInput<'_, AS::Key, AS::Value>
    ) -> RemoveAccessResult {
        self.accesses.remove_access(remove_access_input)
     }
}