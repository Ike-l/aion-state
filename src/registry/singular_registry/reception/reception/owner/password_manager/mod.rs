use tracing::span;

use crate::prelude::{FUNCTION_LEVEL, PasswordManagerAccessPermissionInput, PasswordManagerAccessPermissionResult, PasswordStorage};

pub mod password_storage;

pub mod password_manager_result;
pub mod password_manager_input;

pub struct PasswordManager<PS> {
    password_storage: PS
}

impl<PS: PasswordStorage> PasswordManager<PS> {
    pub fn check_password(
        &self,
        PasswordManagerAccessPermissionInput {
            password, access
        }: PasswordManagerAccessPermissionInput<PS::Password, PS::Access>
    ) -> PasswordManagerAccessPermissionResult {
        let span = span!(FUNCTION_LEVEL, "Password Manager Checking Password");
        let _enter = span.enter();
        
        PasswordManagerAccessPermissionResult::Checked(self.password_storage.check(password, access))
    }

    // generate password
}