use crate::prelude::{ManualRegistryAccessInput, ReceptionAccessPermissionInput};

pub struct SingularRegistryAccessInput<'a, Access, Key> {
    pub access: &'a Access,
    pub key: &'a Key,
}

impl<'a, Access, Key> SingularRegistryAccessInput<'a, Access, Key> {
    pub fn split(self) -> (
        ManualRegistryAccessInput<'a, Access, Key>,
        ReceptionAccessPermissionInput,
    ) {
        (
            ManualRegistryAccessInput {
                access: self.access,
                key: self.key
            },
            ReceptionAccessPermissionInput {
                
            }
        )
    }
}