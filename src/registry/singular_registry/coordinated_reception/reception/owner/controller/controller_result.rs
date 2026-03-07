use crate::prelude::ResourceControlOwnResult;

pub enum ControllerOwnResult {
    ResourceControl(ResourceControlOwnResult)
}