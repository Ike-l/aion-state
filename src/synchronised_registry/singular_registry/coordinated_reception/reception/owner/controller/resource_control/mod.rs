use crate::prelude::{ControlStorage, ResourceControlCheckOwner, ResourceControlCheckOwnerResult, ResourceControlCheckOwnersResult, ResourceControlIsOwnedResult, ResourceControlOwn, ResourceControlOwnResult, ResourceControlRelease, ResourceControlReleaseId, ResourceControlReleaseIdResult, ResourceControlReleaseResult, ResourceIsOwned, trace_function};

pub mod control_storage;

pub mod resource_control_input;
pub mod resource_control_result;

#[derive(Default)]
pub struct ResourceControl<CS> {
    control_storage: CS
}

impl<
    CS: ControlStorage
> ResourceControl<CS> {
    /// If `resource_id` is not currently owned will attempt to claim ownership
    pub fn own(
        &mut self,
        ResourceControlOwn {
            id, resource_id
        }: ResourceControlOwn<CS::Id, CS::ResourceId>
    ) -> ResourceControlOwnResult {
        trace_function!("Resource Control Own");

        if self.control_storage.is_owned(&resource_id) {
            return ResourceControlOwnResult::OwnershipConflict
        }

        ResourceControlOwnResult::Own(self.control_storage.own(id, resource_id))
    }

    pub fn is_owned(
        &self,
        ResourceIsOwned {
            resource_id
        }: &ResourceIsOwned<CS::ResourceId>
    ) -> ResourceControlIsOwnedResult {
        trace_function!("Resource Control Is Owned");
        
        ResourceControlIsOwnedResult::IsOwned(self.control_storage.is_owned(&resource_id))
    }

    /// Passes through to `control_storage`
    pub fn release(
        &mut self,
        ResourceControlRelease {
            resource_id
        }: &ResourceControlRelease<'_, CS::ResourceId>
    ) -> ResourceControlReleaseResult {
        trace_function!("Resource Control Release");

        ResourceControlReleaseResult::Released(self.control_storage.release(resource_id))
    }

    /// Passes through to `control_storage`
    pub fn release_id(
        &mut self,
        ResourceControlReleaseId {
            id
        }: &ResourceControlReleaseId<'_, CS::Id>
    ) -> ResourceControlReleaseIdResult<impl Iterator<Item = CS::ResourceId>> {
        trace_function!("ResourceControl ReleaseId");

        ResourceControlReleaseIdResult::Released(self.control_storage.release_id(id))
    }

    /// Passes through to `control_storage`
    pub fn check_owner(
        &self,
        ResourceControlCheckOwner {
            id, resource_id
        }: &ResourceControlCheckOwner<'_, CS::Id, CS::ResourceId>
    ) -> ResourceControlCheckOwnerResult {
        trace_function!("Resource Control Check Owner");

        ResourceControlCheckOwnerResult::Verification(self.control_storage.check_owner(id, resource_id))
    }

    pub fn check_owners<'a>(
        &self,
        mut inputs: impl Iterator<Item = ResourceControlCheckOwner<'a, CS::Id, CS::ResourceId>>
    ) -> ResourceControlCheckOwnersResult<'a, CS::Id, CS::ResourceId>  {
        trace_function!("Resource Control Check Owners");

        ResourceControlCheckOwnersResult::Invalid(inputs.find(|input| !self.check_owner(input).ok()))
    }
}