use crate::prelude::{PasswordManagerAccessPermissionInput, PasswordManagerAccessPermissionResult, PasswordStorage};

pub mod password_storage;

pub mod password_manager_result;
pub mod password_manager_input;

pub struct PasswordManager<PS> {
    password_storage: PS
}

impl<PS: PasswordStorage> PasswordManager<PS> {
    pub fn permits_access(
        &self,
        PasswordManagerAccessPermissionInput {
            password, access
        }: PasswordManagerAccessPermissionInput<PS::Password, PS::Access>
    ) -> PasswordManagerAccessPermissionResult {
        PasswordManagerAccessPermissionResult::Checked(self.password_storage.check(password, access))
    }

    // generate password
}