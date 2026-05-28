use crate::prelude::{ManualRegistryAccessResult, ManualRegistryReleaseResult, ManualRegistryReplacementResult, ReceptionBlacklistAllowResult, ReceptionBlacklistUnallowResult, ReceptionCheckAccessResult, ReceptionDrainReservationsResult, ReceptionOwnResult, ReceptionRegisterResult, ReceptionReleaseAccessResult, ReceptionReleaseResourceAllResult, ReceptionReleaseResourceResult, ReceptionReservationResult, ReceptionUnregisterResult, ReceptionUnreserveResult, ReceptionUpdatePasswordResult, ReceptionWhitelistAllowResult, ReceptionWhitelistUnallowResult};

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

pub enum SingularRegistryReleaseResourceAllResult {
    Reception(ReceptionReleaseResourceAllResult)
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
    /// bool 
    /// 
    /// True if resource exists
    /// 
    /// False if resource does not exist
    AutomatedRegistry(bool)
}

pub enum SingularRegistryReleaseAccessResult {
    Reception(ReceptionReleaseAccessResult),
    AutomatedRegistry(ManualRegistryReleaseResult)
}

pub enum SingularRegistryReservationResult {
    Reception(ReceptionReservationResult)
}

pub enum SingularRegistryUnreserveResult {
    Reception(ReceptionUnreserveResult)
}

pub enum SingularRegistryDrainReservationsResult<T> {
    Reception(ReceptionDrainReservationsResult<T>)
}

pub enum SingularRegistryAcquireAccessResult {
    AutomatedRegistry(ManualRegistryAccessResult),
    Reception(ReceptionCheckAccessResult)
}

pub enum SingularRegistrySaferReplacementResult<ReplacementResult> {
    AutomatedRegistry(ManualRegistryReplacementResult<ReplacementResult>),
    Reception(ReceptionCheckAccessResult)
}

pub enum SingularRegistryContainsResourceResult {
    AutomatedRegistry(bool)
}