use crate::prelude::{DoorAccessPermissionResult, DoorPermitsAccessInput, LockStorage, Locker, LockerPermitsAccessInput, PasswordManager, PasswordManagerAccessPermissionInput, PasswordStorage, trace_function};

pub mod password_manager;
pub mod locker;

pub mod door_input;
pub mod door_result;

pub struct Door<PS, LS> {
    locker: Locker<LS>,
    password_manager: PasswordManager<PS>,
}

impl<
    PS: PasswordStorage, 
    LS: LockStorage
> Door<PS, LS> {
    pub fn permits_access(
        &self,
        DoorPermitsAccessInput {
            item, password, access
        }: DoorPermitsAccessInput<LS::Item, PS::Password, PS::Access>
    ) -> DoorAccessPermissionResult {
        trace_function!("Door Permits Access");

        if self.locker.check_locked(LockerPermitsAccessInput { item }).ok() {
            DoorAccessPermissionResult::Locked(self.password_manager.check_password(PasswordManagerAccessPermissionInput { password, access }))
        } else {
            DoorAccessPermissionResult::Unlocked   
        }
    }
}