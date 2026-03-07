use crate::prelude::{ControlStorage, ResourceControlRelease, ResourceControlReleaseResult, ResourceControlVerification, ResourceControlVerificationResult};

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

    pub fn release(
        &self,
        ResourceControlRelease {
            id, resource_id
        }: ResourceControlRelease<'_, CS::Id, CS::ResourceId>
    ) -> ResourceControlReleaseResult {
        ResourceControlReleaseResult::Released(self.control_storage.release(id, resource_id))
    }

    pub fn verify(
        &self,
        ResourceControlVerification {
            id, resource_id
        }: ResourceControlVerification<'_, CS::Id, CS::ResourceId>
    ) -> ResourceControlVerificationResult {
        ResourceControlVerificationResult::Verification(self.control_storage.verify(id, resource_id))
    }
}