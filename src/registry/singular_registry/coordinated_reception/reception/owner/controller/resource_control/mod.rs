use crate::prelude::{ControlStorage, ResourceControlOwn, ResourceControlOwnResult, ResourceControlRelease, ResourceControlReleaseResult, ResourceControlVerification, ResourceControlVerificationResult};

pub mod control_storage;

pub mod resource_control_input;
pub mod resource_control_result;

pub struct ResourceControl<CS> {
    control_storage: CS
}

impl<
    CS: ControlStorage
> ResourceControl<CS> {
    pub fn own(
        &self,
        ResourceControlOwn {
            id, resource_id
        }: ResourceControlOwn<CS::Id, CS::ResourceId>
    ) -> ResourceControlOwnResult {
        if self.control_storage.is_owned(&resource_id) {
            return ResourceControlOwnResult::OwnershipConflict
        }

        ResourceControlOwnResult::Own(self.control_storage.own(id, resource_id))
    }

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