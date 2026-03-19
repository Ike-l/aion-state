use crate::prelude::{OwnerRegisterResult, OwnerUnregisterResult};

pub enum ReceptionRegisterResult {
    Owner(OwnerRegisterResult)
}

pub enum ReceptionUnregisterResult {
    Owner(OwnerUnregisterResult)
}