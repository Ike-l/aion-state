use crate::prelude::{PasswordCheckResult, PasswordGeneratorResult};

pub enum DoorAccessPermissionResult {
    Locked(PasswordCheckResult),
    Unlocked
}

impl DoorAccessPermissionResult {
    pub fn ok(&self) -> bool {
        match self {
            DoorAccessPermissionResult::Locked(password_manager_access_permission_result) => password_manager_access_permission_result.ok(),
            DoorAccessPermissionResult::Unlocked => true,
        }
    }
}

pub enum DoorGeneratePasswordResult<Password> {
    PasswordManagerResult(PasswordGeneratorResult<Password>),
    Unlocked,
}