use crate::prelude::{OwnerOwnResult, OwnerRegisterResult, OwnerReleaseResourceResult, OwnerUnregisterResult, OwnerUpdatePasswordResult};

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