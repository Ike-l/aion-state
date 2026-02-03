use crate::prelude::{PasswordGeneratorResult, DoorAccessPermissionResult};

pub enum OwnerAccessPermissionResult {
    Door(DoorAccessPermissionResult),
    OwnerVerified,
    NoCredentials,
}

impl OwnerAccessPermissionResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::OwnerVerified => true,
            Self::Door(password_result) => password_result.ok(),
            Self::NoCredentials => false,
        }
    }
}

pub enum OwnerPasswordGeneratorResult<Password> {
    Generated(PasswordGeneratorResult<Password>),
    Denied,
}