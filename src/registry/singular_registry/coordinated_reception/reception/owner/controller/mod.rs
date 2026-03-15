use crate::prelude::{AccessControl, AccessControlCheckAccess, AccessControlAllow, AccessControlRelease, BlacklistStorage, ControlStorage, ControllerCheckAccess, ControllerCheckAccessResult, ControllerAllow, ControllerBlacklistAllowResult, ControllerOwn, ControllerOwnResult, ControllerReleaseResource, ControllerReleaseId, ControllerReleaseIdResult, ControllerReleaseResourceResult, ControllerWhitelistAllowResult, ResourceControl, ResourceControlOwn, ResourceControlRelease, ResourceControlReleaseId, ResourceControlReleaseIdResult, ResourceControlCheckOwner, WhitelistStorage, trace_function};

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
        trace_function!("Controller Own");

        ControllerOwnResult::ResourceControl(self.resource_control.own(ResourceControlOwn { id, resource_id }))
    }

    /// Releases ownership of a resource
    /// 
    /// And all allowances
    pub fn release_resource(
        &mut self,
        ControllerReleaseResource {
            id, resource_id
        }: ControllerReleaseResource<CS::Id, CS::ResourceId>
    ) -> ControllerReleaseResourceResult {
        trace_function!("Controller Release Resource");

        let access_control_release_result = self.access_control.release(AccessControlRelease { id: resource_id });
        if access_control_release_result.ok() {
            return ControllerReleaseResourceResult::ResourceControl(self.resource_control.release(ResourceControlRelease { id, resource_id }))
        }
        ControllerReleaseResourceResult::AccessControl(access_control_release_result)
    }

    /// If `id` owns `resource_id` then allow `access` on whitelist
    pub fn allow_whitelist(
        &mut self,
        ControllerAllow {
            id, resource_id, access
        }: ControllerAllow<'_, CS::Id, CS::ResourceId, WS::Access>
    ) -> ControllerWhitelistAllowResult {
        trace_function!("Controller Allow Whitelist");

        if self.resource_control.check_owner(ResourceControlCheckOwner { id, resource_id: &resource_id }).ok() {
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
        trace_function!("Controller Allow Blacklist");

        if self.resource_control.check_owner(ResourceControlCheckOwner { id, resource_id: &resource_id }).ok() {
            ControllerBlacklistAllowResult::Blacklist(self.access_control.allow_blacklist(AccessControlAllow { id: resource_id, access }))
        } else {
            ControllerBlacklistAllowResult::Denied
        }
    }
    
    /// If is owner OR password matches then return ok?
    /// 
    // Could open the verify api then drop the `id` branch?
    pub fn check_access(
        &self,
        ControllerCheckAccess {
            id, resource_id, access, password
        }: ControllerCheckAccess<'_, CS::Id, WS::Id, WS::Access, BS::Password>
    ) -> ControllerCheckAccessResult {
        trace_function!("Controller Check Access");

        if let Some(id) = id {
            return ControllerCheckAccessResult::Verification(self.resource_control.check_owner(ResourceControlCheckOwner { id, resource_id }))
        }
        
        ControllerCheckAccessResult::AccessControl(self.access_control.check_access(AccessControlCheckAccess { id: resource_id, access, password }))
    }

    /// Release all resources associated with `id`
    /// 
    /// Release all `accesses` associated with all the resources released in the above
    pub fn release_id(
        &mut self,
        ControllerReleaseId {
            id
        }: ControllerReleaseId<'_, CS::Id> 
    ) -> ControllerReleaseIdResult {
        trace_function!("Controller Release Id");

        let resources = self.resource_control.release_id(ResourceControlReleaseId { id });
        match resources {
            ResourceControlReleaseIdResult::Released(resources) => {
                let resources = resources.collect::<Vec<_>>();
                let release_inputs = resources.iter().map(|resource_id| AccessControlRelease { id: resource_id });
                return ControllerReleaseIdResult::AccessControl(self.access_control.release_all(release_inputs.collect()));
            },
        }
    }

    pub fn unallow_whitelist() {}
    pub fn unallow_blacklist() {}
    pub fn release_all() {}
    pub fn check_owner() {} // ?
}