use crate::prelude::{LockInput, LockResult, LockStorage, LockerAccessPermissionResult, LockerPermitsAccessInput, UnlockInput, UnlockResult, trace_function};

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
        }: LockerPermitsAccessInput<'_, LS::ValueId>
    ) -> LockerAccessPermissionResult {
        trace_function!("Locker Check Locked");

        LockerAccessPermissionResult::LockedResult(self.lock_storage.check(item))
    }

    pub fn lock(
        &self,
        LockInput {
            item
        }: LockInput<LS::ValueId>
    ) -> LockResult{
        trace_function!("Locker Lock");

        LockResult::LockResult(self.lock_storage.lock(item))
    }

    pub fn unlock(
        &self,
        UnlockInput {
            item
        }: UnlockInput<LS::ValueId>
    ) -> UnlockResult{
        trace_function!("Locker Unlock");

        UnlockResult::UnlockResult(self.lock_storage.unlock(item))
    }
}