use crate::prelude::ControllerOwnResult;

pub enum OwnerOwnResult {
    Controller(ControllerOwnResult),
    Denied,
}