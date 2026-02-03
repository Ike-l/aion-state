use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, FUNCTION_LEVEL, Host, HostAccessPermissionInput, Owner, OwnerAccessPermissionInput, OwnerStorage, PasswordStorage, ReceptionAccessPermissionInput, ReceptionAccessPermissionResult, ReservationStorage, trace_function};

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
        trace_function!("Reception Permits Access");

        let owner_access_permission = self.owner.permits_access(OwnerAccessPermissionInput { owner_credentials, password, access });
        if owner_access_permission.ok() {
            ReceptionAccessPermissionResult::Host(self.host.permits_access(HostAccessPermissionInput { reserver, access_key, access }))
        } else {
            ReceptionAccessPermissionResult::Denied
        }
    }

    pub fn generate_password(
        &mut self
    ) {
        trace_function!("Reception Generate Password");
    }
}