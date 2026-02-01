use std::fmt::Debug;

use tracing::span;

use crate::prelude::{AccessStorage, Accessor, FUNCTION_LEVEL, Host, HostAccessPermissionInput, Owner, OwnerAccessPermissionInput, OwnerStorage, ReceptionAccessPermissionInput, ReceptionAccessPermissionResult, ReservationStorage};

pub mod host;
pub mod owner;

pub mod reception_input;
pub mod reception_result;

pub struct Reception<RS, AS, OS> {
    owner: Owner<OS>,
    host: Host<RS, AS>
}

impl<
    RS: ReservationStorage<AccessStorage = AS>, 
    AS: AccessStorage + Default,
    OS: OwnerStorage
> Reception<RS, AS, OS> 
    where 
        RS::Key: Debug + PartialEq,
        AS::Value: Debug + Accessor
{
    pub fn permits_access(
        &self,
        ReceptionAccessPermissionInput {
            reserver, access_key, access, owner_credentials
        }: ReceptionAccessPermissionInput<'_, RS::Key, AS::Key, AS::Value, OS::Key, OS::Value>
    ) -> ReceptionAccessPermissionResult {
        let span = span!(FUNCTION_LEVEL, "Reception Permits Access");
        let _enter = span.enter();

        let owner_access_permission = self.owner.permits_access(OwnerAccessPermissionInput { owner_credentials });
        if owner_access_permission.ok() {
            ReceptionAccessPermissionResult::Host(self.host.permits_access(HostAccessPermissionInput { reserver, access_key, access }))
        } else {
            ReceptionAccessPermissionResult::Denied
        }
    }
}