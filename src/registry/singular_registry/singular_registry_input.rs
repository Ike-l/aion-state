use crate::prelude::{ManualRegistryAccessInput, ReceptionAccessPermissionInput};

pub struct SingularRegistryAccessInput<'a, Access, Key> {
    _a: &'a Access,
    _b: &'a Key
}

impl<'a, Access, Key> SingularRegistryAccessInput<'a, Access, Key> {
    pub fn split(self) -> (
        ManualRegistryAccessInput<'a, Access, Key>,
        ReceptionAccessPermissionInput,
    ) {
        todo!()
    }
}