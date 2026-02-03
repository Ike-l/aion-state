use crate::prelude::PasswordManagerAccessPermissionResult;

pub enum DoorAccessPermissionResult {
    Locked(PasswordManagerAccessPermissionResult),
    Unlocked
}

impl DoorAccessPermissionResult {
    pub fn ok(&self) -> bool {
        todo!()
    }
}