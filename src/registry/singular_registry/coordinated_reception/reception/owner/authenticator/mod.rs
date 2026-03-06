use crate::prelude::{AuthenticateInput, AuthenticationResult, CredentialStorage, trace_function};

pub mod credential_storage;

pub mod authenticator_input;
pub mod authenticator_result;

/// wraps `owner storage` and `ownership storage`
/// 
/// applies `owner storage` semantics and then `ownership storage` semantics
pub struct Authenticator<OS> {
    credentials: OS,
}

impl<
    OS: CredentialStorage,
> Authenticator<OS> {
    /// Authenticates by verifying the `Id` matches the `Password`
    pub fn authenticate(
        &self,
        AuthenticateInput {
            id, password
        }: AuthenticateInput<'_, OS::Id, OS::Password> 
    ) -> AuthenticationResult {
        trace_function!("Authenticating");

        AuthenticationResult::Verification(self.credentials.verify(id, password))
    }
}