use crate::prelude::{AuthenticateInput, Authenticator, OwnerAccessPermissionInput, OwnerAccessPermissionResult, OwnerStorage, PasswordManager};

pub mod authenticator;
pub mod password_manager;

pub mod owner_result;
pub mod owner_input;

pub struct Owner<OS> {
    authenticator: Authenticator<OS>,
    password_manager: PasswordManager
}

impl<OS: OwnerStorage> Owner<OS> {
    pub fn permits_access(
        &self,
        OwnerAccessPermissionInput {
            owner_credentials
        }: OwnerAccessPermissionInput<'_, OS::Key, OS::Value>
    ) -> OwnerAccessPermissionResult {
        if let Some((owner_id, owner_key)) = owner_credentials {
            if self.authenticator.authenticate(AuthenticateInput { owner_id, owner_key }) {
                return OwnerAccessPermissionResult::OwnerVerified;
            }
        }

        OwnerAccessPermissionResult::PasswordResult(self.password_manager.permits_access())
    }
}