use crate::prelude::{ManualRegistryCheckAccessResult, ReceptionBlacklistAllowResult, ReceptionBlacklistUnallowResult, ReceptionCheckAccessResult, ReceptionOwnResult, ReceptionRegisterResult, ReceptionReleaseResourceAllResult, ReceptionReleaseResourceResult, ReceptionUnregisterResult, ReceptionUpdatePasswordResult, ReceptionWhitelistAllowResult, ReceptionWhitelistUnallowResult};

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

pub enum SingularRegistryBlacklistAllowResult<Password> {
    Reception(ReceptionBlacklistAllowResult<Password>)
}

pub enum SingularRegistryWhitelistAllowResult {
    Reception(ReceptionWhitelistAllowResult)
}

pub enum SingularRegistryBlacklistUnallowResult {
    Reception(ReceptionBlacklistUnallowResult)
}

pub enum SingularRegistryWhitelistUnallowResult {
    Reception(ReceptionWhitelistUnallowResult)
}

pub enum SingularRegistryCheckAccessResult {
    Reception(ReceptionCheckAccessResult),
    AutomatedRegistry(ManualRegistryCheckAccessResult)
}