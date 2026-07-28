use crate::prelude::{ManualRegistryAccessError, ManualRegistryReplacementResult, ReceptionBlacklistAllowResult, ReceptionBlacklistUnallowResult, ReceptionCheckAccessResult, ReceptionDrainReservationsResult, ReceptionOwnResult, ReceptionRegisterResult, ReceptionReleaseAccessResult, ReceptionReleaseResourceAllResult, ReceptionReleaseResourceResult, ReceptionReservationResult, ReceptionUnregisterResult, ReceptionUnreserveResult, ReceptionUpdatePasswordResult, ReceptionWhitelistAllowResult, ReceptionWhitelistUnallowResult};

pub enum UnsynchronisedRegistryRegisterResult {
    Reception(ReceptionRegisterResult)
}

pub enum UnsynchronisedRegistryUnregisterResult {
    Reception(ReceptionUnregisterResult)
}

pub enum UnsynchronisedRegistryUpdatePasswordResult {
    Reception(ReceptionUpdatePasswordResult)
}

pub enum UnsynchronisedRegistryOwnResult {
    Reception(ReceptionOwnResult)
}

pub enum UnsynchronisedRegistryReleaseResourceResult {
    Reception(ReceptionReleaseResourceResult)
}

pub enum UnsynchronisedRegistryReleaseResourceAllResult {
    Reception(ReceptionReleaseResourceAllResult)
}

pub enum UnsynchronisedRegistryBlacklistAllowResult<Password> {
    Reception(ReceptionBlacklistAllowResult<Password>)
}

pub enum UnsynchronisedRegistryWhitelistAllowResult {
    Reception(ReceptionWhitelistAllowResult)
}

pub enum UnsynchronisedRegistryBlacklistUnallowResult {
    Reception(ReceptionBlacklistUnallowResult)
}

pub enum UnsynchronisedRegistryWhitelistUnallowResult {
    Reception(ReceptionWhitelistUnallowResult)
}

pub enum UnsynchronisedRegistryCheckAccessResult {
    Reception(ReceptionCheckAccessResult),
    /// bool 
    /// 
    /// True if resource exists
    /// 
    /// False if resource does not exist
    AutomatedRegistry(bool)
}

pub enum UnsynchronisedRegistryReleaseAccessResult {
    Reception(ReceptionReleaseAccessResult),
}

pub enum UnsynchronisedRegistryReservationResult {
    Reception(ReceptionReservationResult)
}

pub enum UnsynchronisedRegistryUnreserveResult {
    Reception(ReceptionUnreserveResult)
}

pub enum UnsynchronisedRegistryDrainReservationsResult<T> {
    Reception(ReceptionDrainReservationsResult<T>)
}

pub enum UnsynchronisedRegistryAcquireAccessError {
    AutomatedRegistry(ManualRegistryAccessError),
    Reception(ReceptionCheckAccessResult)
}

pub enum UnsynchronisedRegistrySaferReplacementResult<ReplacementResult> {
    AutomatedRegistry(ManualRegistryReplacementResult<ReplacementResult>),
    Reception(ReceptionCheckAccessResult)
}

pub enum UnsynchronisedRegistryContainsResourceResult {
    AutomatedRegistry(bool)
}