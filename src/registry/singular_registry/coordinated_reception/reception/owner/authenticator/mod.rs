use crate::prelude::{AuthenticateInput, AuthenticationResult, OwnerStorage, OwnershipStorage, trace_function};

pub mod owner_storage;

pub mod authenticator_input;
pub mod authenticator_result;
pub mod ownership_storage;

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
    pub fn authenticate(
        &self,
        AuthenticateInput {
            owner_id, owner_password, value_id
        }: AuthenticateInput<'_, OS::OwnerId, OS::OwnerPassword, OSS::ValueId> 
    ) -> AuthenticationResult {
        trace_function!("Authenticating Owner");

        match (
            self.owner_storage.verify(owner_id, owner_password),
            self.ownership_storage.owns(owner_id, value_id)
        ) {
            (true, true) => AuthenticationResult::Ok,
            (true, false) => AuthenticationResult::OwnershipError,
            (false, true) => AuthenticationResult::OwnerError,
            (false, false) => AuthenticationResult::Denied,
        }
    }
}