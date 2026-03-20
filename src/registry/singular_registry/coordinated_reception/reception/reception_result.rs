use crate::prelude::{HostCheckAccessResult, HostRecordAccessResult, HostReleaseAccessResult, OwnerAuthenticationResult, OwnerBlacklistAllowResult, OwnerBlacklistUnallowResult, OwnerCheckAccessResult, OwnerOwnResult, OwnerRegisterResult, OwnerReleaseResourceAll, OwnerReleaseResourceAllResult, OwnerReleaseResourceResult, OwnerUnregisterResult, OwnerUpdatePasswordResult, OwnerWhitelistAllowResult, OwnerWhitelistUnallowResult};

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

pub enum ReceptionReleaseResourceAllResult<'a, Id, ResourceId> {
    Owner(OwnerReleaseResourceAllResult<'a, Id, ResourceId>)
}

pub enum ReceptionCheckAccessResult {
    Host(HostCheckAccessResult),
    Denied(OwnerCheckAccessResult)
}

pub enum ReceptionReleaseAccessResult {
    Host(HostReleaseAccessResult),
}

pub enum ReceptionRecordAccessResult {
    Host(HostRecordAccessResult),
    Denied(OwnerCheckAccessResult)
}