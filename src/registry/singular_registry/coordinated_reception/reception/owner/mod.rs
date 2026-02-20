use crate::prelude::{AuthenticateInput, Authenticator, Door, DoorGeneratePasswordInput, DoorPermitsAccessInput, LockStorage, OwnerAccessPermissionInput, OwnerAccessPermissionResult, OwnerPasswordGeneratorInput, OwnerPasswordGeneratorResult, OwnerStorage, OwnershipStorage, PasswordStorage, trace_function};

pub mod authenticator;
pub mod door;

pub mod owner_result;
pub mod owner_input;

// move Door to after Host?

/// Applies `Authentication` semantics when ownership of the door is required, then `Door` semantics 
pub struct Owner<OS, PS, LS, OSS> {
    authenticator: Authenticator<OS, OSS>,
    door: Door<PS, LS>
}

impl<
    OS: OwnerStorage,
    PS: PasswordStorage,
    LS: LockStorage,
    OSS: OwnershipStorage<OwnerId = OS::OwnerId, ValueId = LS::ValueId>,
> Owner<OS, PS, LS, OSS> {
    /// No `authenticator` semantics are need so is a `pass through` function to `Door`
    pub fn permits_access(
        &self,
        OwnerAccessPermissionInput {
            value_id, value_password, access
        }: OwnerAccessPermissionInput<'_, LS::ValueId, PS::ValuePassword, PS::Access>
    ) -> OwnerAccessPermissionResult {
        trace_function!("Owner Permits Access");

        OwnerAccessPermissionResult::Door(self.door.permits_access(DoorPermitsAccessInput { value_id, value_password, access }))

    }

    /// If the caller is authenticated then generate the password
    pub fn generate_password(
        &mut self,
        OwnerPasswordGeneratorInput {
            owner_id, owner_password, value_id, access, policy
        }: OwnerPasswordGeneratorInput<'_, OS::OwnerId, OS::OwnerPassword, LS::ValueId, PS::Access, PS::GenerationPolicy>
    ) -> OwnerPasswordGeneratorResult<PS::ValuePassword> {
        trace_function!("Owner Generates Password");

        if self.authenticate(AuthenticateInput { owner_id, owner_password, value_id }).ok() {
            OwnerPasswordGeneratorResult::Door(self.door.generate_password(DoorGeneratePasswordInput { value_id, access, policy }))
        } else {
            OwnerPasswordGeneratorResult::Denied
        }
    }

    pub fn authenticate(
        &self,
        OwnerAuthenticationInput {

        }: OwnerAuthenticationInput
    ) {
        self.authenticator.authenticate(AuthenticateInput { owner_id, owner_password, value_id })
    }
}