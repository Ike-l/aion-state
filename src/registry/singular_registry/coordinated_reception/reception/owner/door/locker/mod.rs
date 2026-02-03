use crate::prelude::{LockStorage, LockerAccessPermissionResult, LockerPermitsAccessInput, trace_function};

pub mod lock_storage;

pub mod locker_input;
pub mod locker_result;

pub struct Locker<LS> {
    lock_storage: LS
}

impl<
    LS: LockStorage
> Locker<LS> {
    pub fn check_locked(
        &self,
        LockerPermitsAccessInput {
            item
        }: LockerPermitsAccessInput<'_, LS::Item>
    ) -> LockerAccessPermissionResult {
        trace_function!("Locker Check Locked");

        LockerAccessPermissionResult::LockedResult(self.lock_storage.check(item))
    }

    // pub fn lock(
    //     &self
    // )

    // pub fn unlock(
    //     &self
    // )
}