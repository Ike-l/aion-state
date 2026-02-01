use crate::prelude::{Authenticator, OwnerAccessPermissionResult, OwnerStorage, PasswordManager};

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
        &self
    ) -> OwnerAccessPermissionResult {
        if self.authenticator.authenticate(authenticate_input) {

        } else {
            if self.password_manager.permits_access() {

            } else {
                OwnerAccessPermissionResult::Denied
            }
        }
        // if owner
        // or if password is allowed
    }
}