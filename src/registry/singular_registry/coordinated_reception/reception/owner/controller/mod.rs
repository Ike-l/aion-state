use crate::prelude::{AccessControl, AccessControlAccess, AccessControlAllow, AccessControlRelease, BlacklistStorage, ControlStorage, ControllerAccess, ControllerAccessResult, ControllerAllow, ControllerBlacklistAllowResult, ControllerOwn, ControllerOwnResult, ControllerRelease, ControllerReleaseId, ControllerReleaseIdResult, ControllerReleaseResult, ControllerWhitelistAllowResult, ResourceControl, ResourceControlOwn, ResourceControlRelease, ResourceControlReleaseId, ResourceControlReleaseIdResult, ResourceControlVerification, WhitelistStorage};

pub mod access_control;
pub mod resource_control;

pub mod controller_input;
pub mod controller_result;

/// Applies `Access Control` & `Resource Control` semantics
/// 
/// `Resource Control` links an `id` with a `resource id`
/// 
/// `Access Control` forms `resource id` `access` pairs with an optional password
pub struct Controller<WS, BS, CS> {
    access_control: AccessControl<WS, BS>,
    resource_control: ResourceControl<CS>
}

impl<
    WS: WhitelistStorage,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<ResourceId = WS::Id>,
> Controller<WS, BS, CS> {
    /// Asserts ownership of a resource if it doesnt have an owner already
    ///
    /// doesn't need to check access_control because cannot have access control on a resource that isn't owned
    pub fn own(
        &mut self,
        ControllerOwn {
            id, resource_id
        }: ControllerOwn<CS::Id, CS::ResourceId>
    ) -> ControllerOwnResult {
        ControllerOwnResult::ResourceControl(self.resource_control.own(ResourceControlOwn { id, resource_id }))
    }

    /// Releases ownership of a resource
    /// 
    /// And all allowances
    pub fn release(
        &mut self,
        ControllerRelease {
            id, resource_id
        }: ControllerRelease<CS::Id, CS::ResourceId>
    ) -> ControllerReleaseResult {
        let access_control_release_result = self.access_control.release(AccessControlRelease { id: resource_id });
        if access_control_release_result.ok() {
            return ControllerReleaseResult::ResourceControl(self.resource_control.release(ResourceControlRelease { id, resource_id }))
        }
        ControllerReleaseResult::AccessControl(access_control_release_result)
    }

    /// If `id` owns `resource_id` then allow `access` on whitelist
    pub fn allow_whitelist(
        &mut self,
        ControllerAllow {
            id, resource_id, access
        }: ControllerAllow<'_, CS::Id, CS::ResourceId, WS::Access>
    ) -> ControllerWhitelistAllowResult {
        if self.resource_control.verify(ResourceControlVerification { id, resource_id: &resource_id }).ok() {
            ControllerWhitelistAllowResult::Whitelist(self.access_control.allow_whitelist(AccessControlAllow { id: resource_id, access }))
        } else {
            ControllerWhitelistAllowResult::Denied
        }
    }

    /// If `id` owns `resource_id` then allow `access` on blacklist 
    pub fn allow_blacklist(
        &mut self,
        ControllerAllow {
            id, resource_id, access
        }: ControllerAllow<'_, CS::Id, CS::ResourceId, BS::Access>
    ) -> ControllerBlacklistAllowResult<BS::Password> {
        if self.resource_control.verify(ResourceControlVerification { id, resource_id: &resource_id }).ok() {
            ControllerBlacklistAllowResult::Blacklist(self.access_control.allow_blacklist(AccessControlAllow { id: resource_id, access }))
        } else {
            ControllerBlacklistAllowResult::Denied
        }
    }
    
    /// If is owner OR password matches then return ok?
    /// 
    // Could open the verify api then drop the `id` branch?
    pub fn access(
        &self,
        ControllerAccess {
            id, resource_id, access, password
        }: ControllerAccess<'_, CS::Id, WS::Id, WS::Access, BS::Password>
    ) -> ControllerAccessResult {
        if let Some(id) = id {
            return ControllerAccessResult::Verification(self.resource_control.verify(ResourceControlVerification { id, resource_id }))
        }
        
        ControllerAccessResult::AccessControl(self.access_control.access(AccessControlAccess { id: resource_id, access, password }))
    }

    pub fn release_id(
        &mut self,
        ControllerReleaseId {
            id
        }: ControllerReleaseId<'_, CS::Id> 
    ) -> ControllerReleaseIdResult {
        let resources = self.resource_control.release_id(ResourceControlReleaseId { id });
        match resources {
            ResourceControlReleaseIdResult::Released(resources) => {
                let resources = resources.collect::<Vec<_>>();
                let release_inputs = resources.iter().map(|resource_id| AccessControlRelease { id: resource_id });
                return ControllerReleaseIdResult::AccessControl(self.access_control.release_all(release_inputs.collect()));
            },
        }
    }
}