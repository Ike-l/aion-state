use crate::prelude::{PasswordGeneratorResult, PasswordManagerAccessPermissionResult};

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

pub enum OwnerPasswordGeneratorResult<Password> {
    Generated(PasswordGeneratorResult<Password>),
    Denied,
}