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
        }: &Authentication<'_, CS::Id, CS::Password> 
    ) -> AuthenticationResult {
        trace_function!("Authenticator Authenticating");

        AuthenticationResult::Verification(self.credentials.verify(id, password))
    }

    /// Register a user
    /// 
    /// Such that `authenticate` with the same `id` & `password` returns true
    pub fn register(
        &mut self,
        AuthenticateRegister {
            id, password
        }: AuthenticateRegister<CS::Id, CS::Password>
    ) -> AuthenticateRegistrationResult {
        trace_function!("Authenticator Registering");

        AuthenticateRegistrationResult::Registration(self.credentials.register(id, password))
    }

    /// Update the password
    /// 
    /// Assumes the caller has the authority to do so
    pub fn update_password(
        &mut self,
        AuthenticateUpdatePassword {
            id, new_password
        }: AuthenticateUpdatePassword<CS::Id, CS::Password>
    ) -> AuthenticateUpdatePasswordResult {
        trace_function!("Authenticator Updating Password");

        AuthenticateUpdatePasswordResult::Updated(self.credentials.update_password(id, new_password))
    }

    /// Unregister a user
    /// 
    /// Acts as the opposite to `register`
    pub fn unregister(
        &mut self,
        AuthenticateUnregister {
            id
        }: &AuthenticateUnregister<'_, CS::Id>
    ) -> AuthenticateUnregisterResult {
        trace_function!("Authenticator Unregistering");
        
        AuthenticateUnregisterResult::Unregister(self.credentials.unregister(id))
    }
}