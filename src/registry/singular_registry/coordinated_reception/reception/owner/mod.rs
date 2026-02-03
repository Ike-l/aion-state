use crate::prelude::{AuthenticateInput, Authenticator, Door, DoorGeneratePasswordInput, DoorPermitsAccessInput, LockStorage, OwnerAccessPermissionInput, OwnerAccessPermissionResult, OwnerPasswordGeneratorInput, OwnerPasswordGeneratorResult, OwnerStorage, PasswordStorage, trace_function};

pub mod authenticator;
pub mod door;

pub mod owner_result;
pub mod owner_input;

pub struct Owner<OS, PS, LS> {
    authenticator: Authenticator<OS>,
    door: Door<PS, LS>
}

impl<
    OS: OwnerStorage,
    PS: PasswordStorage,
    LS: LockStorage
> Owner<OS, PS, LS> {
    pub fn permits_access(
        &self,
        OwnerAccessPermissionInput {
            owner_credentials, item, password, access
        }: OwnerAccessPermissionInput<'_, OS::Key, OS::Value, LS::Item, PS::Password, PS::Access>
    ) -> OwnerAccessPermissionResult {
        trace_function!("Owner Permits Access");

        if let Some((owner_id, owner_key)) = owner_credentials {
            if self.authenticator.authenticate(AuthenticateInput { owner_id, owner_key }) {
                return OwnerAccessPermissionResult::OwnerVerified;
            }
        }

        OwnerAccessPermissionResult::Door(self.door.permits_access(DoorPermitsAccessInput { item, password, access }))

    }

    pub fn generate_password(
        &mut self,
        OwnerPasswordGeneratorInput {
            owner_id, owner_key, item, access, policy
        }: OwnerPasswordGeneratorInput<'_, OS::Key, OS::Value, LS::Item, PS::Access, PS::GenerationPolicy>
    ) -> OwnerPasswordGeneratorResult<PS::Password> {
        trace_function!("Owner Generates Password");

        if self.authenticator.authenticate(AuthenticateInput { owner_id, owner_key }) {
            OwnerPasswordGeneratorResult::Door(self.door.generate_password(DoorGeneratePasswordInput { item, access, policy }))
        } else {
            OwnerPasswordGeneratorResult::Denied
        }
    }
}