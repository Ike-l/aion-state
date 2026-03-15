use crate::prelude::{AccessControl, AccessControlAllow, AccessControlCheckAccess, AccessControlRelease, AccessControlUnallow, BlacklistStorage, ControlStorage, ControllerAllow, ControllerBlacklistAllowResult, ControllerCheckAccess, ControllerCheckAccessResult, ControllerCheckOwner, ControllerCheckOwnerResult, ControllerOwn, ControllerOwnResult, ControllerReleaseId, ControllerReleaseIdResult, ControllerReleaseResource, ControllerReleaseResourceResult, ControllerUnallow, ControllerUnallowBlacklistResult, ControllerUnallowWhitelistResult, ControllerWhitelistAllowResult, ResourceControl, ResourceControlCheckOwner, ResourceControlOwn, ResourceControlRelease, ResourceControlReleaseId, ResourceControlReleaseIdResult, WhitelistStorage, trace_function};

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

    pub fn unallow_whitelist(
        &mut self,
        ControllerUnallow {
            id, resource_id, access
        }: ControllerUnallow<'_, CS::Id, CS::ResourceId, WS::Access>
    ) -> ControllerUnallowWhitelistResult {
        trace_function!("Controller Unallow Whitelist");

        if self.resource_control.check_owner(ResourceControlCheckOwner { id, resource_id }).ok() {
            return ControllerUnallowWhitelistResult::Whitelist(self.access_control.unallow_whitelist(AccessControlUnallow { id: resource_id, access }))
        }

        ControllerUnallowWhitelistResult::Denied
    }

    pub fn unallow_blacklist(
        &mut self,
        ControllerUnallow {
            id, resource_id, access
        }: ControllerUnallow<'_, CS::Id, CS::ResourceId, BS::Access>
    ) -> ControllerUnallowBlacklistResult {
        trace_function!("Controller Unallow Blacklist");

        if self.resource_control.check_owner(ResourceControlCheckOwner { id, resource_id }).ok() {
            return ControllerUnallowBlacklistResult::Blacklist(self.access_control.unallow_blacklist(AccessControlUnallow { id: resource_id, access }))
        }

        ControllerUnallowBlacklistResult::Denied
    }

    pub fn release_all(
        &mut self,
    ) {}

    pub fn check_owner(
        &self,
        ControllerCheckOwner {
            
        }: ControllerCheckOwner
    ) -> ControllerCheckOwnerResult {

    }
}