use crate::prelude::{DoorAccessPermissionResult, DoorGeneratePasswordInput, DoorGeneratePasswordResult, DoorPermitsAccessInput, LockStorage, PasswordStorage, trace_function};

pub mod password_storage;
pub mod lock_storage;

pub mod door_input;
pub mod door_result;

/// Wraps `lock storage` 
/// 
/// Applies `lock storage` semantics and then `password storage` semantics
/// 
/// If lock storage semantics are a boolean truth about whether a value can be accessed
/// 
/// Then password semantics allow a spectrum of permissions over a locked value 
pub struct Door<PS, LS> {
    lock_storage: LS,
    password_storage: PS,
}

impl<
    PS: PasswordStorage, 
    LS: LockStorage
> Door<PS, LS> {
    /// Is the value currently locked;
    /// 
    /// If so: Is the incoming password permitted
    pub fn permits_access(
        &self,
        DoorPermitsAccessInput {
            value_id, value_password, access
        }: DoorPermitsAccessInput<LS::ValueId, PS::ValuePassword, PS::Access>
    ) -> DoorAccessPermissionResult {
        trace_function!("Door Permits Access");

        if self.lock_storage.check(value_id) {
            DoorAccessPermissionResult::Locked(self.password_storage.check(value_password, access))
        } else {
            DoorAccessPermissionResult::Unlocked   
        }
    }
    // permits_access -> check_value_password
    // check_reserver_password -> when generating reservations make a password for that reserver

    /// If the value is locked then generate a password for the access & policy
    /// 
    /// Otherwise do nothing: Only locked values can be given a password
    pub fn generate_password(
        &mut self,
        DoorGeneratePasswordInput {
            value_id, access, policy
        }: DoorGeneratePasswordInput<LS::ValueId, PS::Access, PS::GenerationPolicy>
    ) -> DoorGeneratePasswordResult<PS::ValuePassword> {
        if self.lock_storage.check(value_id) {
            DoorGeneratePasswordResult::PasswordManagerResult(self.password_storage.generate_password(access, policy))
        } else {
            DoorGeneratePasswordResult::Unlocked
        }
    }
}