use crate::prelude::{AccessControl, BlacklistStorage, ControlStorage, ControllerOwn, ControllerOwnResult, ResourceControl, ResourceControlOwn, WhitelistStorage};

pub mod access_control;
pub mod resource_control;

pub mod controller_input;
pub mod controller_result;

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
        &self
    ) {}

    /// If own a resource then allow certain 
    pub fn allow() {}
    
    pub fn access() {}
}