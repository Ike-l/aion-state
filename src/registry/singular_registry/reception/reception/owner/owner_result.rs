use crate::prelude::PasswordManagerAccessPermissionResult;

pub enum OwnerAccessPermissionResult {
    PasswordResult(PasswordManagerAccessPermissionResult),
    OwnerVerified,
    NoCredentials,
}

impl OwnerAccessPermissionResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::OwnerVerified => true,
            Self::PasswordResult(password_result) => password_result.ok(),
            Self::NoCredentials => false,
        }
    }
}