use crate::prelude::{OwnerRegisterResult, OwnerUnregisterResult, OwnerUpdatePasswordResult};

pub enum ReceptionRegisterResult {
    Owner(OwnerRegisterResult)
}

pub enum ReceptionUnregisterResult {
    Owner(OwnerUnregisterResult)
}

pub enum ReceptionUpdatePasswordResult {
    Owner(OwnerUpdatePasswordResult)
}