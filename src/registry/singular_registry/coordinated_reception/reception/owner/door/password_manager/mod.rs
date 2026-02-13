use crate::prelude::{PasswordGeneratorInput, PasswordGeneratorResult, PasswordManagerAccessPermissionInput, PasswordCheckResult, PasswordStorage, trace_function};

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
        }: PasswordManagerAccessPermissionInput<PS::ValuePassword, PS::Access>
    ) -> PasswordCheckResult {
        trace_function!("Checking Password");
        
        PasswordCheckResult::Checked(self.password_storage.check(password, access))
    }

    pub fn generate_password(
        &mut self,
        PasswordGeneratorInput {
            access, policy
        }: PasswordGeneratorInput<'_, PS::Access, PS::GenerationPolicy>
    ) -> PasswordGeneratorResult<PS::ValuePassword> {
        trace_function!("Generating Password");

        PasswordGeneratorResult::Generated(self.password_storage.generate_password(access, policy))
    }
}