use crate::prelude::{Reception, ReceptionAccessPermission, ReceptionAccessPermissionInput, ReceptionRecordAccessInput};

pub mod coordinated_reception_result;
pub mod coordinated_reception_input;

pub mod reception;

pub struct CoordinatedReception<RS, AS> {
    reception: parking_lot::RwLock<Reception<RS, AS>>
}

impl<S, R> CoordinatedReception<S, R> {
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