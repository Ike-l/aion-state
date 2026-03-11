use crate::prelude::{AuthenticateRegister, AuthenticateRegistrationResult, AuthenticateUnregister, AuthenticateUnregisterResult, AuthenticateUpdatePassword, AuthenticateUpdatePasswordResult, Authentication, AuthenticationResult, CredentialStorage, trace_function};

pub mod credential_storage;

pub mod authenticator_input;
pub mod authenticator_result;

/// wraps `credential storage`
pub struct Authenticator<CS> {
    credentials: CS,
}

impl<
    CS: CredentialStorage,
> Authenticator<CS> {
    /// Authenticates by verifying the `Id` matches the `Password`
    pub fn authenticate(
        &self,
        Authentication {
            id, password
        }: Authentication<'_, CS::Id, CS::Password> 
    ) -> AuthenticationResult {
        trace_function!("Authenticating");

        AuthenticationResult::Verification(self.credentials.verify(id, password))
    }

    pub fn register(
        &mut self,
        AuthenticateRegister {
            id, password
        }: AuthenticateRegister<CS::Id, CS::Password>
    ) -> AuthenticateRegistrationResult {

        AuthenticateRegistrationResult::Registration(self.credentials.register(id, password))
    }

    pub fn update_password(
        &mut self,
        AuthenticateUpdatePassword {
            id, new_password
        }: AuthenticateUpdatePassword<CS::Id, CS::Password>
    ) -> AuthenticateUpdatePasswordResult {
        AuthenticateUpdatePasswordResult::Updated(self.credentials.update_password(id, new_password))
    }

    pub fn unregister(
        &mut self,
        AuthenticateUnregister {
            id
        }: AuthenticateUnregister<'_, CS::Id>
    ) -> AuthenticateUnregisterResult {
        AuthenticateUnregisterResult::Unregister(self.credentials.unregister(id))
    }
}