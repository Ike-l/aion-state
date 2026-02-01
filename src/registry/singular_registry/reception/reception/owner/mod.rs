use tracing::span;

use crate::prelude::{AuthenticateInput, Authenticator, FUNCTION_LEVEL, OwnerAccessPermissionInput, OwnerAccessPermissionResult, OwnerStorage, PasswordManager, PasswordManagerAccessPermissionInput, PasswordStorage};

pub mod authenticator;
pub mod password_manager;

pub mod owner_result;
pub mod owner_input;

pub struct Owner<OS, PS> {
    authenticator: Authenticator<OS>,
    password_manager: PasswordManager<PS>
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
        let span = span!(FUNCTION_LEVEL, "Owner Permits Access");
        let _enter = span.enter();

        if let Some((owner_id, owner_key)) = owner_credentials {
            if self.authenticator.authenticate(AuthenticateInput { owner_id, owner_key }) {
                return OwnerAccessPermissionResult::OwnerVerified;
            }
        }

        if let Some(password) = password {
            OwnerAccessPermissionResult::PasswordResult(self.password_manager.check_password(PasswordManagerAccessPermissionInput { password, access }))
        } else {
            OwnerAccessPermissionResult::NoCredentials
        }
    }

    pub fn generate_password(
        &mut self
    ) {
        let span = span!(FUNCTION_LEVEL, "Owner Generating Password");
        let _enter = span.enter();

        if self.authenticator.authenticate(AuthenticateInput { owner_id, owner_key }) {
            return self.password_manager.generate_password(access)
        } else {

        }
    }
    // generate password
    // check if owner authenticate
    // then password_manager.generate
}