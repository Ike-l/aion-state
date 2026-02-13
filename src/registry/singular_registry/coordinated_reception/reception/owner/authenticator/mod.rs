use crate::prelude::{AuthenticateInput, OwnerStorage, OwnershipStorage, trace_function};

pub mod owner_storage;

pub mod authenticator_input;
pub mod ownership_storage;

pub struct Authenticator<OS, OSS> {
    // if owner_storage stores owner id: password then
    owner_storage: OS,
    ownership_storage: OSS,
    // x stores owner id: resource id
    // Item, Id
}

impl<
    OS: OwnerStorage,
    OSS: OwnershipStorage<OwnerId = OS::OwnerId>
> Authenticator<OS, OSS> {
    pub fn authenticate(
        &self,
        AuthenticateInput {
            owner_id, owner_key // item
        }: AuthenticateInput<'_, OS::OwnerId, OS::OwnerPassword> // OS::Item
    ) -> bool {
        trace_function!("Authenticating Owner");
        

        // if:
        self.owner_storage.verify(owner_id, owner_key)
        // && self.ownership_storage.owns(item)
    }
}