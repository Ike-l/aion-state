pub enum LockerAccessPermissionResult {
    LockedResult(bool)
}

impl LockerAccessPermissionResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::LockedResult(true))
    }
}

pub enum LockResult {
    LockResult(bool)
}

pub enum UnlockResult {
    UnlockResult(bool)
}