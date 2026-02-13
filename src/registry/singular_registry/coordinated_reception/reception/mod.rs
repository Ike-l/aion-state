use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, Host, HostAccessPermissionInput, LockStorage, Owner, OwnerAccessPermissionInput, OwnerStorage, OwnershipStorage, PasswordStorage, ReceptionAccessPermissionInput, ReceptionAccessPermissionResult, ReservationStorage, trace_function};

pub mod host;
pub mod owner;

pub mod reception_input;
pub mod reception_result;

pub struct Reception<RS, AS, OS, PS, LS, OSS> {
    owner: Owner<OS, PS, LS, OSS>,
    host: Host<RS, AS>
}

impl<
    RS: ReservationStorage<AccessStorage = AS>, 
    AS: AccessStorage + Default,
    OS: OwnerStorage,
    OSS: OwnershipStorage<ValueId = LS::ValueId, OwnerId = OS::OwnerId>,
    PS: PasswordStorage<Access = AS::Access>,
    LS: LockStorage<ValueId = AS::AccessId>,
> Reception<RS, AS, OS, PS, LS, OSS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor,
{
    pub fn permits_access(
        &self,
        ReceptionAccessPermissionInput {
            reserver, access_key, access, owner_credentials, password
        }: ReceptionAccessPermissionInput<'_, RS::ReserverId, AS::AccessId, AS::Access, OS::OwnerId, OS::OwnerPassword, PS::ValuePassword>
    ) -> ReceptionAccessPermissionResult {
        trace_function!("Reception Permits Access");

        let owner_access_permission = self.owner.permits_access(OwnerAccessPermissionInput { owner_credentials, item: access_key, password, access });
        if owner_access_permission.ok() {
            ReceptionAccessPermissionResult::Host(self.host.permits_access(HostAccessPermissionInput { reserver, access_key, access }))
        } else {
            ReceptionAccessPermissionResult::Denied
        }
    }

    // pub fn locks
    // checks for any reservation or accesses 
    // if !self.host.has_stakeholder {
    //  self.owner.lock()
    // }
    // if self.host.allows_ownership
    // if !(self.host.has_reservations or self.host.has_access)

    pub fn generate_password(
        &mut self
    ) {
        trace_function!("Reception Generate Password");

        // this will check if its locked :)
        // self.owner.generate_password(owner_password_generator_input)
    }
}