use std::fmt::Debug;

use tracing::span;

use crate::prelude::{AccessStorage, Accessor, FUNCTION_LEVEL, Host, HostAccessPermissionInput, Owner, OwnerAccessPermissionInput, OwnerStorage, PasswordStorage, ReceptionAccessPermissionInput, ReceptionAccessPermissionResult, ReservationStorage};

pub mod host;
pub mod owner;

pub mod reception_input;
pub mod reception_result;

pub struct Reception<RS, AS, OS, PS> {
    owner: Owner<OS, PS>,
    host: Host<RS, AS>
}

impl<
    RS: ReservationStorage<AccessStorage = AS>, 
    AS: AccessStorage + Default,
    OS: OwnerStorage,
    PS: PasswordStorage<Access = AS::Value>
> Reception<RS, AS, OS, PS> 
    where 
        RS::Key: Debug + PartialEq,
        AS::Value: Debug + Accessor,
{
    pub fn permits_access(
        &self,
        ReceptionAccessPermissionInput {
            reserver, access_key, access, owner_credentials, password
        }: ReceptionAccessPermissionInput<'_, RS::Key, AS::Key, AS::Value, OS::Key, OS::Value, PS::Password>
    ) -> ReceptionAccessPermissionResult {
        let span = span!(FUNCTION_LEVEL, "Reception Permits Access");
        let _enter = span.enter();

        let owner_access_permission = self.owner.permits_access(OwnerAccessPermissionInput { owner_credentials, password, access });
        if owner_access_permission.ok() {
            ReceptionAccessPermissionResult::Host(self.host.permits_access(HostAccessPermissionInput { reserver, access_key, access }))
        } else {
            ReceptionAccessPermissionResult::Denied
        }
    }
}