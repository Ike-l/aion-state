use crate::prelude::{AuthenticateInput, OwnerStorage, trace_function};

pub mod owner_storage;

pub mod authenticator_input;

pub struct Authenticator<OS> {
    // if owner_storage stores owner id: password then
    owner_storage: OS,
    ownership_storage: OSS,
    // x stores owner id: resource id
    // Item, Id
}

impl<
    OS: OwnerStorage
    // OSS<Id: OS::Key>
> Authenticator<OS> {
    pub fn authenticate(
        &self,
        AuthenticateInput {
            owner_id, owner_key // item
        }: AuthenticateInput<'_, OS::Key, OS::Value> // OS::Item
    ) -> bool {
        trace_function!("Authenticating Owner");
        

        // if:
        self.owner_storage.verify(owner_id, owner_key)
        // && self.ownership_storage.owns(item)
    }
}