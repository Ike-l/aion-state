use crate::prelude::{AccessControlReleaseAllResult, AuthenticateRegistrationResult, AuthenticateUnregisterResult, AuthenticateUpdatePasswordResult, AuthenticationResult, BlacklistReleaseAllResult, ControllerOwnResult, ControllerReleaseIdResult, OwnerOwnResult, OwnerRegisterResult, OwnerUnregisterResult, OwnerUpdatePasswordResult, ReceptionOwnResult, ReceptionRegisterResult, ReceptionUnregisterResult, ReceptionUpdatePasswordResult, ResourceControlOwnResult, SingularRegistryOwnResult, SingularRegistryRegisterResult, SingularRegistryUnregisterResult, SingularRegistryUpdatePasswordResult, WhitelistReleaseAllResult};

pub enum RegistryRegisterResult {
    Ok,
    Err
}

impl From<SingularRegistryRegisterResult> for RegistryRegisterResult {
    fn from(value: SingularRegistryRegisterResult) -> Self {
        match value {
            SingularRegistryRegisterResult::Reception(reception_register_result) => {
                match reception_register_result {
                    ReceptionRegisterResult::Owner(owner_register_result) => {
                        match owner_register_result {
                            OwnerRegisterResult::Authenticator(authenticate_registration_result) => {
                                match authenticate_registration_result {
                                    AuthenticateRegistrationResult::Registration(result) => {
                                        match result {
                                            true => Self::Ok,
                                            false => Self::Err,
                                        }
                                    },
                                }
                            },
                        }
                    },
                }
            },
        }
    }
}

pub enum RegistryUnregisterResult {
    Ok,
    Err,
    Lists{whitelist_result: bool, blacklist_result: bool},
    VerificationFailure
}

impl From<SingularRegistryUnregisterResult> for RegistryUnregisterResult {
    fn from(value: SingularRegistryUnregisterResult) -> Self {
        match value {
            SingularRegistryUnregisterResult::Reception(reception_unregister_result) => {
                match reception_unregister_result {
                    ReceptionUnregisterResult::Owner(owner_unregister_result) => {
                        match owner_unregister_result {
                            OwnerUnregisterResult::Controller(controller_release_id_result) => {
                                match controller_release_id_result {
                                    ControllerReleaseIdResult::AccessControl(access_control_release_all_result) => {
                                        match access_control_release_all_result {
                                            AccessControlReleaseAllResult::Lists((whitelist, blacklist)) => {
                                                match (whitelist, blacklist) {
                                                    (WhitelistReleaseAllResult::Release(whitelist_result), BlacklistReleaseAllResult::Release(blacklist_result)) => {
                                                        Self::Lists{whitelist_result, blacklist_result}
                                                    },
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            OwnerUnregisterResult::Authenticator(authenticate_unregister_result) => {
                                match authenticate_unregister_result {
                                    AuthenticateUnregisterResult::Unregister(result) => {
                                        match result {
                                            true => Self::Ok,
                                            false => Self::Err,
                                        }
                                    },
                                }
                            },
                            OwnerUnregisterResult::Denied(authentication_result) => {
                                match authentication_result {
                                    AuthenticationResult::Verification(result) => {
                                        assert_eq!(result, false);
                                        Self::VerificationFailure
                                    },
                                }
                            },
                        }
                    },
                }
            },
        }
    }
}

pub enum RegistryUpdatePasswordResult {
    Ok,
    Err,
    VerificationFailure
}

impl From<SingularRegistryUpdatePasswordResult> for RegistryUpdatePasswordResult {
    fn from(value: SingularRegistryUpdatePasswordResult) -> Self {
        match value {
            SingularRegistryUpdatePasswordResult::Reception(reception_update_password_result) => {
                match reception_update_password_result {
                    ReceptionUpdatePasswordResult::Owner(owner_update_password_result) => {
                        match owner_update_password_result {
                            OwnerUpdatePasswordResult::Authenticator(authenticate_update_password_result) => {
                                match authenticate_update_password_result {
                                    AuthenticateUpdatePasswordResult::Updated(result) => {
                                        match result {
                                            true => Self::Ok,
                                            false => Self::Err,
                                        }
                                    },
                                }
                            },
                            OwnerUpdatePasswordResult::Denied(authentication_result) => {
                                match authentication_result {
                                    AuthenticationResult::Verification(result) => {
                                        assert_eq!(result, false);

                                        Self::VerificationFailure
                                    },
                                }
                            },
                        }
                    },
                }
            },
        }
    }
}
pub enum RegistryOwnResult {
    Ok,
    Err,
    OwnershipConflict,
    VerificationFailure
}

impl From<SingularRegistryOwnResult> for RegistryOwnResult {
    fn from(value: SingularRegistryOwnResult) -> Self {
        match value {
            SingularRegistryOwnResult::Reception(reception_own_result) => {
                match reception_own_result {
                    ReceptionOwnResult::Owner(owner_own_result) => {
                        match owner_own_result {
                            OwnerOwnResult::Controller(controller_own_result) => {
                                match controller_own_result {
                                    ControllerOwnResult::ResourceControl(resource_control_own_result) => {
                                        match resource_control_own_result {
                                            ResourceControlOwnResult::Own(result) => {
                                                match result {
                                                    true => Self::Ok,
                                                    false => Self::Err,
                                                }
                                            },
                                            ResourceControlOwnResult::OwnershipConflict => {
                                                Self::OwnershipConflict
                                            },
                                        }
                                    },
                                }
                            },
                            OwnerOwnResult::Denied(authentication_result) => {
                                match authentication_result {
                                    AuthenticationResult::Verification(result) => {
                                        assert_eq!(result, false);

                                        Self::VerificationFailure
                                    },
                                }
                            },
                        }
                    },
                }
            },
        }
    }
}

pub enum RegistryReleaseResourceResult {

}

pub enum RegistryReleaseResourceAllResult {

}

pub enum RegistryBlacklistAllowResult {

}

pub enum RegistryWhitelistAllowResult {

}

pub enum RegistryBlacklistUnallowResult {

}

pub enum RegistryWhitelistUnallowResult {

}

pub enum RegistryCheckAccessResult {

}

pub enum RegistryReleaseAccessResult {

}

pub enum RegistryReservationResult {

}

pub enum RegistryUnreserveResult {

}

pub enum RegistryDrainReservationsResult {

}

pub enum RegistryAcquireAccessResult {

}

pub enum RegistrySaferReplacementResult {

}

pub enum RegistryContainsResourceResult {

}