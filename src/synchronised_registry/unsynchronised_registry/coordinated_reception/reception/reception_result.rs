use crate::prelude::{HostCheckAccessResult, HostDrainReservationsResult, HostRecordAccessResult, HostReleaseAccessResult, HostReservationResult, HostUnreserveResult, OwnerAuthenticationResult, OwnerBlacklistAllowResult, OwnerBlacklistUnallowResult, OwnerCheckAccessResult, OwnerOwnResult, OwnerRegisterResult, OwnerReleaseResourceAllResult, OwnerReleaseResourceResult, OwnerUnregisterResult, OwnerUpdatePasswordResult, OwnerWhitelistAllowResult, OwnerWhitelistUnallowResult};

pub enum ReceptionRegisterResult {
    Owner(OwnerRegisterResult)
}

pub enum ReceptionUnregisterResult {
    Owner(OwnerUnregisterResult)
}

pub enum ReceptionUpdatePasswordResult {
    Owner(OwnerUpdatePasswordResult)
}

pub enum ReceptionOwnResult {
    Owner(OwnerOwnResult)
}

pub enum ReceptionReleaseResourceResult {
    Owner(OwnerReleaseResourceResult)
}

pub enum ReceptionWhitelistAllowResult {
    Owner(OwnerWhitelistAllowResult)
}

pub enum ReceptionBlacklistAllowResult<Password> {
    Owner(OwnerBlacklistAllowResult<Password>)
}

pub enum ReceptionWhitelistUnallowResult {
    Owner(OwnerWhitelistUnallowResult)
}

pub enum ReceptionBlacklistUnallowResult {
    Owner(OwnerBlacklistUnallowResult)
}

pub enum ReceptionReleaseResourceAllResult {
    Owner(OwnerReleaseResourceAllResult)
}

pub enum ReceptionCheckAccessResult {
    Host(HostCheckAccessResult),
    Denied(OwnerCheckAccessResult)
}

impl ReceptionCheckAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Host(host) => host.ok(),
            _ => false
        }
    }
}

pub enum ReceptionReleaseAccessResult {
    Host(HostReleaseAccessResult),
}

#[derive(Debug)]
pub enum ReceptionRecordAccessResult {
    Host(HostRecordAccessResult),
    Denied(OwnerCheckAccessResult)
}

impl ReceptionRecordAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Host(host) => host.ok(),
            _ => false
        }
    }
}

pub enum ReceptionReservationResult {
    Host(HostReservationResult),
    Denied(OwnerCheckAccessResult),
}

pub enum ReceptionUnreserveResult {
    Host(HostUnreserveResult),
    VerificationFailure
}

pub enum ReceptionDrainReservationsResult<T> {
    Host(HostDrainReservationsResult<T>),
    Denied(OwnerAuthenticationResult)
}