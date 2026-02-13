use crate::prelude::{AuthenticateInput, Authenticator, Door, DoorGeneratePasswordInput, DoorPermitsAccessInput, LockStorage, OwnerAccessPermissionInput, OwnerAccessPermissionResult, OwnerPasswordGeneratorInput, OwnerPasswordGeneratorResult, OwnerStorage, OwnershipStorage, PasswordStorage, trace_function};

pub mod authenticator;
pub mod door;

pub mod owner_result;
pub mod owner_input;

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
    pub fn permits_access(
        &self,
        OwnerAccessPermissionInput {
            owner_credentials, value_id, value_password, access
        }: OwnerAccessPermissionInput<'_, OS::OwnerId, OS::OwnerPassword, LS::ValueId, PS::ValuePassword, PS::Access>
    ) -> OwnerAccessPermissionResult {
        trace_function!("Owner Permits Access");

        if let Some((owner_id, owner_password)) = owner_credentials {
            let authentication_result = self.authenticator.authenticate(AuthenticateInput { owner_id, owner_password, value_id });
            if !authentication_result.ok() {
                return OwnerAccessPermissionResult::AuthenticationError(authentication_result)
            }
        }

        OwnerAccessPermissionResult::Door(self.door.permits_access(DoorPermitsAccessInput { value_id, value_password, access }))

    }

    pub fn generate_password(
        &mut self,
        OwnerPasswordGeneratorInput {
            owner_id, owner_password, value_id, access, policy
        }: OwnerPasswordGeneratorInput<'_, OS::OwnerId, OS::OwnerPassword, LS::ValueId, PS::Access, PS::GenerationPolicy>
    ) -> OwnerPasswordGeneratorResult<PS::ValuePassword> {
        trace_function!("Owner Generates Password");

        if self.authenticator.authenticate(AuthenticateInput { owner_id, owner_password, value_id }).ok() {
            OwnerPasswordGeneratorResult::Door(self.door.generate_password(DoorGeneratePasswordInput { value_id, access, policy }))
        } else {
            OwnerPasswordGeneratorResult::Denied
        }
    }
}