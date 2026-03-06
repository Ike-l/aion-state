use crate::prelude::{AccessControl, BlacklistStorage, ControlStorage, ResourceControl, WhitelistStorage};

pub mod access_control;
pub mod resource_control;

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
    pub fn own() {}

    /// Releases ownership of a resource
    pub fn release() {}

    /// If own a resource then allow certain 
    pub fn allow() {}
    
    pub fn access() {}
}