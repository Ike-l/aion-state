use crate::prelude::{OwnerBlacklistAllowResult, OwnerBlacklistUnallowResult, OwnerOwnResult, OwnerRegisterResult, OwnerReleaseResourceResult, OwnerUnregisterResult, OwnerUpdatePasswordResult, OwnerWhitelistAllowResult, OwnerWhitelistUnallowResult};

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