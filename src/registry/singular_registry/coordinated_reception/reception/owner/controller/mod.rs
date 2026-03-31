use tracing::event;

use crate::prelude::{AccessControl, AccessControlAllow, AccessControlCheckAccess, AccessControlRelease, AccessControlUnallow, BlacklistStorage, ControlStorage, ControllerAllow, ControllerBlacklistAllowResult, ControllerBlacklistUnallowResult, ControllerCheckAccess, ControllerCheckAccessResult, ControllerCheckOwner, ControllerCheckOwnerResult, ControllerOwn, ControllerOwnResult, ControllerReleaseId, ControllerReleaseIdResult, ControllerReleaseResource, ControllerReleaseResourceAllResult, ControllerReleaseResourceResult, ControllerUnallow, ControllerWhitelistAllowResult, ControllerWhitelistUnallowResult, FUNCTION_LEVEL, ResourceControl, ResourceControlCheckOwner, ResourceControlOwn, ResourceControlRelease, ResourceControlReleaseId, ResourceControlReleaseIdResult, WhitelistStorage, trace_function};

pub mod access_control;
pub mod resource_control;

pub mod controller_input;
pub mod controller_result;

/// Applies `Access Control` & `Resource Control` semantics
/// 
/// `Resource Control` links an `id` with a `resource id`
/// 
/// `Access Control` forms `resource id` `access` pairs with an optional password
#[derive(Default)]
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
        }: &ControllerReleaseResource<'_, CS::Id, CS::ResourceId>
    ) -> ControllerReleaseResourceResult {
        trace_function!("Controller Release Resource");

        let checked_owner = self.resource_control.check_owner(&ResourceControlCheckOwner { id, resource_id });

        if !checked_owner.ok() {
            return ControllerReleaseResourceResult::Denied
        }

        let access_control_release_result = self.access_control.release(&AccessControlRelease { id: resource_id });
        
        event!(FUNCTION_LEVEL, result =? access_control_release_result.ok(), "Access Control Release Result");

        ControllerReleaseResourceResult::ResourceControl(self.resource_control.release(&ResourceControlRelease { resource_id }))
    }

    /// If `id` owns `resource_id` then allow `access` on whitelist
    pub fn allow_whitelist(
        &mut self,
        ControllerAllow {
            id, resource_id, access
        }: ControllerAllow<'_, CS::Id, CS::ResourceId, WS::Access>
    ) -> ControllerWhitelistAllowResult {
        trace_function!("Controller Allow Whitelist");

        if self.resource_control.check_owner(&ResourceControlCheckOwner { id, resource_id: &resource_id }).ok() {
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

        if self.resource_control.check_owner(&ResourceControlCheckOwner { id, resource_id: &resource_id }).ok() {
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
        }: &ControllerCheckAccess<'_, CS::Id, WS::Id, WS::Access, BS::Password>
    ) -> ControllerCheckAccessResult {
        trace_function!("Controller Check Access");

        if let Some(id) = id {
            return ControllerCheckAccessResult::Verification(self.resource_control.check_owner(&ResourceControlCheckOwner { id, resource_id }))
        }
        
        ControllerCheckAccessResult::AccessControl(self.access_control.check_access(&AccessControlCheckAccess { id: resource_id, access, password: *password }))
    }

    /// Release all resources associated with `id`
    /// 
    /// Release all `accesses` associated with all the resources released in the above
    pub fn release_id(
        &mut self,
        ControllerReleaseId {
            id
        }: &ControllerReleaseId<'_, CS::Id> 
    ) -> ControllerReleaseIdResult {
        trace_function!("Controller Release Id");

        let resource_control_input = ResourceControlReleaseId { id: *id };
        let resources = self.resource_control.release_id(&resource_control_input);
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
        }: &ControllerUnallow<'_, CS::Id, CS::ResourceId, WS::Access>
    ) -> ControllerWhitelistUnallowResult {
        trace_function!("Controller Unallow Whitelist");

        if self.resource_control.check_owner(&ResourceControlCheckOwner { id, resource_id }).ok() {
            return ControllerWhitelistUnallowResult::Whitelist(self.access_control.unallow_whitelist(&AccessControlUnallow { id: resource_id, access }))
        }

        ControllerWhitelistUnallowResult::Denied
    }

    pub fn unallow_blacklist(
        &mut self,
        ControllerUnallow {
            id, resource_id, access
        }: &ControllerUnallow<'_, CS::Id, CS::ResourceId, BS::Access>
    ) -> ControllerBlacklistUnallowResult {
        trace_function!("Controller Unallow Blacklist");

        if self.resource_control.check_owner(&ResourceControlCheckOwner { id, resource_id }).ok() {
            return ControllerBlacklistUnallowResult::Blacklist(self.access_control.unallow_blacklist(&AccessControlUnallow { id: resource_id, access }))
        }

        ControllerBlacklistUnallowResult::Denied
    }

    pub fn release_resource_all<'a>(
        &mut self,
        inputs: Vec<&ControllerReleaseResource<'a, CS::Id, CS::ResourceId>>
    ) -> ControllerReleaseResourceAllResult {
        trace_function!("Controller Release Resource All");

        ControllerReleaseResourceAllResult::All(inputs.into_iter().map(|input| self.release_resource(input)).collect())  
    }

    pub fn check_owner(
        &self,
        ControllerCheckOwner {
            id, resource_id
        }: &ControllerCheckOwner<'_, CS::Id, CS::ResourceId>
    ) -> ControllerCheckOwnerResult {
        trace_function!("Controller Check Owner");

        ControllerCheckOwnerResult::ResourceControl(self.resource_control.check_owner(&ResourceControlCheckOwner { id, resource_id }))
    }
}