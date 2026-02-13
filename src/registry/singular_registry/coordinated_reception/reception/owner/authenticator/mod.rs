use crate::prelude::{AuthenticateInput, AuthenticationResult, OwnerStorage, OwnershipStorage, trace_function};

pub mod owner_storage;

pub mod authenticator_input;
pub mod authenticator_result;
pub mod ownership_storage;

/// wraps `owner storage` and `ownership storage`
/// 
/// applies `owner storage` semantics and then `ownership storage` semantics
pub struct Authenticator<OS, OSS> {
    owner_storage: OS,
    ownership_storage: OSS,
}

impl<
    OS: OwnerStorage,
    OSS: OwnershipStorage<OwnerId = OS::OwnerId>
> Authenticator<OS, OSS> {
    /// Are you the owner; 
    /// 
    /// Do you own the resource;
    /// 
    /// Authenticates by verifying with owner storage then ownership storage
    pub fn authenticate(
        &self,
        AuthenticateInput {
            owner_id, owner_password, value_id
        }: AuthenticateInput<'_, OS::OwnerId, OS::OwnerPassword, OSS::ValueId> 
    ) -> AuthenticationResult {
        trace_function!("Authenticating Owner");

        if self.owner_storage.verify(owner_id, owner_password) {
            AuthenticationResult::OwnershipVerification(self.ownership_storage.verify(owner_id, value_id))
        } else {
            AuthenticationResult::Denied
        }
    }
}