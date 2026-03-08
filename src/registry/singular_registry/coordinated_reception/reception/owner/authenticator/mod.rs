use crate::prelude::{AuthenticateInput, AuthenticationResult, CredentialStorage, trace_function};

pub mod credential_storage;

pub mod authenticator_input;
pub mod authenticator_result;

/// wraps `owner storage` and `ownership storage`
/// 
/// applies `owner storage` semantics and then `ownership storage` semantics
pub struct Authenticator<CS> {
    credentials: CS,
}

impl<
    CS: CredentialStorage,
> Authenticator<CS> {
    /// Authenticates by verifying the `Id` matches the `Password`
    pub fn authenticate(
        &self,
        AuthenticateInput {
            id, password
        }: AuthenticateInput<'_, CS::Id, CS::Password> 
    ) -> AuthenticationResult {
        trace_function!("Authenticating");

        AuthenticationResult::Verification(self.credentials.verify(id, password))
    }
}