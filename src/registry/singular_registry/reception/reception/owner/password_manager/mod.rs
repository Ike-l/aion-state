use tracing::span;

use crate::prelude::{FUNCTION_LEVEL, PasswordGeneratorInput, PasswordGeneratorResult, PasswordManagerAccessPermissionInput, PasswordManagerAccessPermissionResult, PasswordStorage};

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

    pub fn generate_password(
        &mut self,
        PasswordGeneratorInput {
            access
        }: PasswordGeneratorInput<'_, PS::Access>
    ) -> PasswordGeneratorResult<PS::Password> {
        let span = span!(FUNCTION_LEVEL, "Password Manager Generating Password");
        let _enter = span.enter();

        PasswordGeneratorResult::Generated(self.password_storage.generate_password(access))
    }
    // generate password
}