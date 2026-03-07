use crate::prelude::{AccessControlReleaseResult, ResourceControlOwnResult, ResourceControlReleaseResult};

pub enum ControllerOwnResult {
    ResourceControl(ResourceControlOwnResult)
}

pub enum ControllerReleaseResult {
    ResourceControl(ResourceControlReleaseResult),
    AccessControl(AccessControlReleaseResult)
}