use crate::prelude::PasswordManagerAccessPermissionResult;

pub enum OwnerAccessPermissionResult {
    OwnerVerified,
    PasswordResult(PasswordManagerAccessPermissionResult)
}

impl OwnerAccessPermissionResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::OwnerVerified => true,
            Self::PasswordResult(password_result) => password_result.ok(),
        }
    }
}