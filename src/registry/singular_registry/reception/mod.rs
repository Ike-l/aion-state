use crate::prelude::{Host, Owner, ReceptionAccessPermission, ReceptionAccessPermissionInput, ReceptionRecordAccessInput};

pub mod owner;
pub mod host;
pub mod reception_result;
pub mod reception_input;

pub struct Reception<S, R> {
    owner: Owner,
    host: Host<S, R>,
}

impl<S, R> Reception<S, R> {
    pub fn permits_access(
        &self,
        _input: &ReceptionAccessPermissionInput
    ) -> ReceptionAccessPermission {
        todo!()
    }

    pub fn record_access(
        &self,
        _input: ReceptionRecordAccessInput
    ) {
        todo!()
    }
}