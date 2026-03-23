use crate::prelude::{ReceptionOwnResult, ReceptionRegisterResult, ReceptionReleaseResourceResult, ReceptionUnregisterResult, ReceptionUpdatePasswordResult};

pub enum SingularRegistryRegisterResult {
    Reception(ReceptionRegisterResult)
}

pub enum SingularRegistryUnregisterResult {
    Reception(ReceptionUnregisterResult)
}

pub enum SingularRegistryUpdatePasswordResult {
    Reception(ReceptionUpdatePasswordResult)
}

pub enum SingularRegistryOwnResult {
    Reception(ReceptionOwnResult)
}

pub enum SingularRegistryReleaseResourceResult {
    Reception(ReceptionReleaseResourceResult)
}