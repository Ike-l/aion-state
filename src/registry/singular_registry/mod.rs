use tracing::span;

use crate::prelude::{Accessor, AutomatedRegistry, FUNCTION_LEVEL, ManualRegistryAccessResult, Reception, ReceptionAccessPermission, ReceptionAccessPermissionInput, ReservationMap, SingularRegistryAccessInput, SingularRegistryAccessResult, Storage};

pub mod automated_registry;
pub mod reception;
pub mod singular_registry_result;
pub mod singular_registry_input;

pub struct SingularRegistry<S, T, R> {
    automated_registry: AutomatedRegistry<S>,
    reception: Reception<T, R>,
}

impl<
    S: Storage,
    T: Storage,
    R: ReservationMap<T>
> SingularRegistry<S, T, R> {
    pub fn permits_access(
        &self,
        input: &ReceptionAccessPermissionInput
    ) -> ReceptionAccessPermission {
        self.reception.permits_access(input)
    }

    pub fn access<
        Access: Accessor<StoredValue = S::Value>
    >(
        &self,
        input: SingularRegistryAccessInput<'_, Access, S::Key>
    ) -> SingularRegistryAccessResult<
        ManualRegistryAccessResult<Access::AccessResult<'_, Access::Value>>, 
        ReceptionAccessPermission,
    > {
        let span = span!(FUNCTION_LEVEL, "Singular Access");
        let _enter = span.enter();

        let (
            access_input,
            permission_input
        ) = input.split();

        let permission = self.permits_access(&permission_input);
        if permission.ok() {
            let access = unsafe { self.automated_registry.access(&access_input) };
            if matches!(access, ManualRegistryAccessResult::Found(_)) {
                self.reception.record_access(permission_input.as_record_access_input());
            }

            return SingularRegistryAccessResult::OkAccess(access);
        } else {
            return SingularRegistryAccessResult::DeniedAccess(permission);
        }
    }
}