use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, Host, HostAccessPermissionInput, LockStorage, Owner, OwnerAccessPermissionInput, OwnerPasswordGeneratorInput, OwnerStorage, OwnershipStorage, PasswordStorage, ReceptionAccessPermissionInput, ReceptionAccessPermissionResult, ReceptionPasswordGeneratorInput, ReservationStorage, trace_function};

pub mod host;
pub mod owner;

pub mod reception_input;
pub mod reception_result;

/// Applies `Owner` semantics, then `Host` semantics
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
    LS: LockStorage<ValueId = AS::ValueId>,
> Reception<RS, AS, OS, PS, LS, OSS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Debug + Accessor,
{
    /// If owner permits then can check if host also permits
    pub fn permits_access(
        &self,
        ReceptionAccessPermissionInput {
            reserver_id, access_id, access, value_password
        }: ReceptionAccessPermissionInput<'_, RS::ReserverId, AS::ValueId, AS::Access, PS::ValuePassword>
    ) -> ReceptionAccessPermissionResult {
        trace_function!("Reception Permits Access");

        let owner_access_permission = self.owner.permits_access(OwnerAccessPermissionInput { value_id: access_id, value_password, access });
        if owner_access_permission.ok() {
            ReceptionAccessPermissionResult::Host(self.host.permits_access(HostAccessPermissionInput { reserver_id, access_id, access }))
        } else {
            ReceptionAccessPermissionResult::Denied
        }
    }

    // if locked then deny reservation without password
    


    // pub fn locks
    // checks for any reservation or accesses 
    // if !self.host.has_stakeholder {
    //  self.owner.lock()
    // }
    // if self.host.allows_ownership
    // if !(self.host.has_reservations or self.host.has_access)


    // /// `pass through` to owner- since owner generates passwords
    // /// 
    // /// Understanding this is as simple as understanding the order of semantics
    // /// 
    // /// Host semantics are after owner the host should not "add" any semantics to the password
    pub fn generate_password(
        &mut self,
        ReceptionPasswordGeneratorInput {
            owner_id, owner_password, value_id, access, policy
        }: ReceptionPasswordGeneratorInput<'_, OS::OwnerId, OS::OwnerPassword, LS::ValueId, PS::Access, PS::GenerationPolicy>
    ) {
        trace_function!("Reception Generate Password");

        // To generate passwords does the `host` have to do anything? 
        // When generating a password iow do i need `reservation` or `accesses` semantics?
        self.owner.generate_password(OwnerPasswordGeneratorInput { owner_id, owner_password, value_id, access, policy });
        todo!("Return result");
        unreachable!()
    }
}