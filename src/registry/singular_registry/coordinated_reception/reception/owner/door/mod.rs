use crate::prelude::{DoorAccessPermissionResult, DoorGeneratePasswordInput, DoorGeneratePasswordResult, DoorPermitsAccessInput, LockStorage, Locker, LockerPermitsAccessInput, PasswordGeneratorInput, PasswordManager, PasswordManagerAccessPermissionInput, PasswordStorage, trace_function};

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
    /// Is the value currently locked;
    /// 
    /// If so: Is the password correct
    pub fn permits_access(
        &self,
        DoorPermitsAccessInput {
            value_id, value_password, access
        }: DoorPermitsAccessInput<LS::ValueId, PS::ValuePassword, PS::Access>
    ) -> DoorAccessPermissionResult {
        trace_function!("Door Permits Access");

        if self.locker.check_locked(LockerPermitsAccessInput { value_id }).ok() {
            DoorAccessPermissionResult::Locked(self.password_manager.check_password(PasswordManagerAccessPermissionInput { value_password, access }))
        } else {
            DoorAccessPermissionResult::Unlocked   
        }
    }

    pub fn generate_password(
        &mut self,
        DoorGeneratePasswordInput {
            value_id, access, policy
        }: DoorGeneratePasswordInput<LS::ValueId, PS::Access, PS::GenerationPolicy>
    ) -> DoorGeneratePasswordResult<PS::ValuePassword> {
        if self.locker.check_locked(LockerPermitsAccessInput { value_id }).ok() {
            DoorGeneratePasswordResult::PasswordManagerResult(self.password_manager.generate_password(PasswordGeneratorInput { access, policy }))
        } else {
            DoorGeneratePasswordResult::Unlocked
        }
    }
}