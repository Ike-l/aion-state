use tracing::span;

use crate::prelude::{AuthenticateInput, FUNCTION_LEVEL, OwnerStorage};

pub mod owner_storage;

pub mod authenticator_input;

pub struct Authenticator<OS> {
    owner_storage: OS
}

impl<OS: OwnerStorage> Authenticator<OS> {
    pub fn authenticate(
        &self,
        AuthenticateInput {
            owner_id, owner_key
        }: AuthenticateInput<'_, OS::Key, OS::Value>
    ) -> bool {
        let span = span!(FUNCTION_LEVEL, "Authenticating Owner");
        let _enter = span.enter();
        
        self.owner_storage.verify(owner_id, owner_key)
    }
}