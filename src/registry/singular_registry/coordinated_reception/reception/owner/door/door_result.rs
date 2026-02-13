pub enum DoorAccessPermissionResult {
    Locked(bool),
    Unlocked
}

impl DoorAccessPermissionResult {
    pub fn ok(&self) -> bool {
        !matches!(self, DoorAccessPermissionResult::Locked(false))
    }
}

pub enum DoorGeneratePasswordResult<Password> {
    PasswordManagerResult(Option<Password>),
    Unlocked,
}