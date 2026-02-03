use crate::prelude::{AuthenticateInput, Authenticator, FUNCTION_LEVEL, OwnerAccessPermissionInput, OwnerAccessPermissionResult, OwnerPasswordGeneratorInput, OwnerPasswordGeneratorResult, OwnerStorage, PasswordGeneratorInput, PasswordManager, PasswordManagerAccessPermissionInput, PasswordStorage, trace_function};

pub mod authenticator;
pub mod password_manager;

pub mod owner_result;
pub mod owner_input;

pub struct Owner<OS, PS> {
    authenticator: Authenticator<OS>,
    password_manager: PasswordManager<PS>
    // door: Door<PS, LS(LockStorage)>
    // door has locks: "Storage"<ResourceId, bool>
}

impl<
    OS: OwnerStorage,
    PS: PasswordStorage
> Owner<OS, PS> {
    pub fn permits_access(
        &self,
        OwnerAccessPermissionInput {
            owner_credentials, password, access
        }: OwnerAccessPermissionInput<'_, OS::Key, OS::Value, PS::Password, PS::Access>
    ) -> OwnerAccessPermissionResult {
        trace_function!("Owner Permits Access");

        if let Some((owner_id, owner_key)) = owner_credentials {
            if self.authenticator.authenticate(AuthenticateInput { owner_id, owner_key }) {
                return OwnerAccessPermissionResult::OwnerVerified;
            }
        }

        OwnerAccessPermissionResult::PasswordResult(self.password_manager.check_password(PasswordManagerAccessPermissionInput { password, access }))
    }

    pub fn generate_password(
        &mut self,
        OwnerPasswordGeneratorInput {
            owner_id, owner_key, access, policy
        }: OwnerPasswordGeneratorInput<'_, OS::Key, OS::Value, PS::Access, PS::GenerationPolicy>
    ) -> OwnerPasswordGeneratorResult<PS::Password> {
        trace_function!("Owner Generates Password");

        if self.authenticator.authenticate(AuthenticateInput { owner_id, owner_key }) {
            OwnerPasswordGeneratorResult::Generated(self.password_manager.generate_password(PasswordGeneratorInput { access, policy }))
        } else {
            OwnerPasswordGeneratorResult::Denied
        }
    }
}