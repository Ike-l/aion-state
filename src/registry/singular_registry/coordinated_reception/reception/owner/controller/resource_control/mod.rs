use crate::prelude::{ControlStorage, ResourceControlVerification, ResourceControlVerificationResult};

pub mod control_storage;

pub mod resource_control_input;
pub mod resource_control_result;

pub struct ResourceControl<CS> {
    control_storage: CS
}

impl<
    CS: ControlStorage
> ResourceControl<CS> {
    pub fn own() {}
    pub fn release() {}
    pub fn verify(
        &self,
        ResourceControlVerification {
            id, resource_id
        }: ResourceControlVerification<'_, CS::Id, CS::ResourceId>
    ) -> ResourceControlVerificationResult {
        ResourceControlVerificationResult::Verification(self.control_storage.verify(id, resource_id))
    }
}