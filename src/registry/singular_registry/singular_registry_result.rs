use crate::prelude::{ReceptionOwnResult, ReceptionRegisterResult, ReceptionReleaseResourceAllResult, ReceptionReleaseResourceResult, ReceptionUnregisterResult, ReceptionUpdatePasswordResult};

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

pub enum SingularRegistryReleaseResourceAllResult<'a, Id, ResourceId> {
    Reception(ReceptionReleaseResourceAllResult<'a, Id, ResourceId>)
}