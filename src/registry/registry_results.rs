use crate::prelude::{AccessControlBlacklistAllowResult, AccessControlBlacklistUnallowResult, AccessControlCheckAccessResult, AccessControlReleaseAllResult, AccessControlWhitelistAllowResult, AccessControlWhitelistUnallowResult, AccessesCheckAccessResult, AccessesDrainResult, AccessesReleaseResult, AuthenticateRegistrationResult, AuthenticateUnregisterResult, AuthenticateUpdatePasswordResult, AuthenticationResult, BlacklistAllowResult, BlacklistCheckAccessResult, BlacklistReleaseAllResult, BlacklistUnallowResult, ControllerBlacklistAllowResult, ControllerBlacklistUnallowResult, ControllerCheckAccessResult, ControllerOwnResult, ControllerReleaseIdResult, ControllerReleaseResourceAllResult, ControllerReleaseResourceResult, ControllerWhitelistAllowResult, ControllerWhitelistUnallowResult, HostCheckAccessResult, HostDrainReservationsResult, HostReleaseAccessResult, HostReservationResult, HostUnreserveResult, ManualRegistryAccessResult, ManualRegistryReleaseResult, ManualRegistryReplacementResult, OwnerAuthenticationResult, OwnerBlacklistAllowResult, OwnerBlacklistUnallowResult, OwnerCheckAccessResult, OwnerOwnResult, OwnerRegisterResult, OwnerReleaseResourceAllResult, OwnerReleaseResourceResult, OwnerUnregisterResult, OwnerUpdatePasswordResult, OwnerWhitelistAllowResult, OwnerWhitelistUnallowResult, ReceptionBlacklistAllowResult, ReceptionBlacklistUnallowResult, ReceptionCheckAccessResult, ReceptionDrainReservationsResult, ReceptionOwnResult, ReceptionRegisterResult, ReceptionReleaseAccessResult, ReceptionReleaseResourceAllResult, ReceptionReleaseResourceResult, ReceptionReservationResult, ReceptionUnregisterResult, ReceptionUnreserveResult, ReceptionUpdatePasswordResult, ReceptionWhitelistAllowResult, ReceptionWhitelistUnallowResult, ReservationsCheckAccessResult, ReservationsDrainReservationsResult, ReservationsReserveResult, ReservationsUnreserveResult, ResourceControlCheckOwnerResult, ResourceControlOwnResult, ResourceControlReleaseResult, SingularRegistryAcquireAccessResult, SingularRegistryBlacklistAllowResult, SingularRegistryBlacklistUnallowResult, SingularRegistryCheckAccessResult, SingularRegistryContainsResourceResult, SingularRegistryDrainReservationsResult, SingularRegistryOwnResult, SingularRegistryRegisterResult, SingularRegistryReleaseAccessResult, SingularRegistryReleaseResourceAllResult, SingularRegistryReleaseResourceResult, SingularRegistryReservationResult, SingularRegistrySaferReplacementResult, SingularRegistryUnregisterResult, SingularRegistryUnreserveResult, SingularRegistryUpdatePasswordResult, SingularRegistryWhitelistAllowResult, SingularRegistryWhitelistUnallowResult, WhitelistAllowResult, WhitelistCheckAccessResult, WhitelistReleaseAllResult, WhitelistUnallowResult};

pub enum RegistryRegisterResult {
    Ok,
    Err
}

impl RegistryRegisterResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Ok)
    }
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
    Ok,
    Err,
    OwnershipDenied,
    VerificationFailure
}

impl RegistryReleaseResourceResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Ok)
    }
}

impl From<SingularRegistryReleaseResourceResult> for RegistryReleaseResourceResult {
    fn from(value: SingularRegistryReleaseResourceResult) -> Self {
        match value {
            SingularRegistryReleaseResourceResult::Reception(reception_release_resource_result) => {
                match reception_release_resource_result {
                    ReceptionReleaseResourceResult::Owner(owner_release_resource_result) => {
                        match owner_release_resource_result {
                            OwnerReleaseResourceResult::Controller(controller_release_resource_result) => {
                                match controller_release_resource_result {
                                    ControllerReleaseResourceResult::ResourceControl(resource_control_release_result) => {
                                        match resource_control_release_result {
                                            ResourceControlReleaseResult::Released(result) => {
                                                match result {
                                                    true => Self::Ok,
                                                    false => Self::Err,
                                                }
                                            },
                                        }
                                    },
                                    ControllerReleaseResourceResult::Denied => {
                                        Self::OwnershipDenied
                                    },
                                }
                            },
                            OwnerReleaseResourceResult::Denied(authentication_result) => {
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

pub enum RegistryReleaseResourceAllResult {
    /// RegistryReleaseResourceResult::VerificationFailure is unreachable!()
    All(Vec<RegistryReleaseResourceResult>),
    VerificationFailure
}

impl From<SingularRegistryReleaseResourceAllResult> for RegistryReleaseResourceAllResult {
    fn from(value: SingularRegistryReleaseResourceAllResult) -> Self {
        match value {
            SingularRegistryReleaseResourceAllResult::Reception(reception_release_resource_all_result) => {
                match reception_release_resource_all_result {
                    ReceptionReleaseResourceAllResult::Owner(owner_release_resource_all_result) => {
                        match owner_release_resource_all_result {
                            OwnerReleaseResourceAllResult::Controller(controller_release_resource_all_result) => {
                                match controller_release_resource_all_result {
                                    ControllerReleaseResourceAllResult::All(controller_release_resource_results) => {
                                        Self::All(controller_release_resource_results.into_iter().map(|result| {
                                            match result {
                                                ControllerReleaseResourceResult::ResourceControl(resource_control_release_result) => {
                                                    match resource_control_release_result {
                                                        ResourceControlReleaseResult::Released(result) => {
                                                            match result {
                                                                true => RegistryReleaseResourceResult::Ok,
                                                                false => RegistryReleaseResourceResult::Err,
                                                            }
                                                        },
                                                    }
                                                },
                                                ControllerReleaseResourceResult::Denied => RegistryReleaseResourceResult::OwnershipDenied,
                                            }
                                        }).collect())
                                    },
                                }
                            },
                            OwnerReleaseResourceAllResult::Denied(authentication_result) => {
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

pub enum RegistryBlacklistAllowResult<Password> {
    Ok(Password),
    Err,
    OwnershipDenied,
    VerificationFailure
}

impl<Password> From<SingularRegistryBlacklistAllowResult<Password>> for RegistryBlacklistAllowResult<Password> {
    fn from(value: SingularRegistryBlacklistAllowResult<Password>) -> Self {
        match value {
            SingularRegistryBlacklistAllowResult::Reception(reception_blacklist_allow_result) => {
                match reception_blacklist_allow_result {
                    ReceptionBlacklistAllowResult::Owner(owner_blacklist_allow_result) => {
                        match owner_blacklist_allow_result {
                            OwnerBlacklistAllowResult::Controller(controller_blacklist_allow_result) => {
                                match controller_blacklist_allow_result {
                                    ControllerBlacklistAllowResult::Blacklist(access_control_blacklist_allow_result) => {
                                        match access_control_blacklist_allow_result {
                                            AccessControlBlacklistAllowResult::Blacklist(blacklist_allow_result) => {
                                                match blacklist_allow_result {
                                                    BlacklistAllowResult::Allow(result) => {
                                                        match result {
                                                            Some(password) => Self::Ok(password),
                                                            None => Self::Err,
                                                        }
                                                    },
                                                }
                                            },
                                        }
                                    },
                                    ControllerBlacklistAllowResult::Denied => {
                                        Self::OwnershipDenied
                                    },
                                }
                            },
                            OwnerBlacklistAllowResult::Denied(authentication_result) => {
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

pub enum RegistryWhitelistAllowResult {
    Ok,
    Err,
    OwnershipDenied,
    VerificationFailure
}

impl From<SingularRegistryWhitelistAllowResult> for RegistryWhitelistAllowResult {
    fn from(value: SingularRegistryWhitelistAllowResult) -> Self {
        match value {
            SingularRegistryWhitelistAllowResult::Reception(reception_whitelist_allow_result) => {
                match reception_whitelist_allow_result {
                    ReceptionWhitelistAllowResult::Owner(owner_whitelist_allow_result) => {
                        match owner_whitelist_allow_result {
                            OwnerWhitelistAllowResult::Controller(controller_whitelist_allow_result) => {
                                match controller_whitelist_allow_result {
                                    ControllerWhitelistAllowResult::Whitelist(access_control_whitelist_allow_result) => {
                                        match access_control_whitelist_allow_result {
                                            AccessControlWhitelistAllowResult::Whitelist(whitelist_allow_result) => {
                                                match whitelist_allow_result {
                                                    WhitelistAllowResult::Allow(result) => {
                                                        match result {
                                                            true => Self::Ok,
                                                            false => Self::Err,
                                                        }
                                                    },
                                                }
                                            },
                                        }
                                    },
                                    ControllerWhitelistAllowResult::Denied => {
                                        Self::OwnershipDenied
                                    },
                                }
                            },
                            OwnerWhitelistAllowResult::Denied(authentication_result) => {
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

pub enum RegistryBlacklistUnallowResult {
    Ok,
    Err,
    OwnershipDenied,
    VerificationFailure
}

impl From<SingularRegistryBlacklistUnallowResult> for RegistryBlacklistUnallowResult {
    fn from(value: SingularRegistryBlacklistUnallowResult) -> Self {
        match value {
            SingularRegistryBlacklistUnallowResult::Reception(reception_blacklist_unallow_result) => {
                match reception_blacklist_unallow_result {
                    ReceptionBlacklistUnallowResult::Owner(owner_blacklist_unallow_result) => {
                        match owner_blacklist_unallow_result {
                            OwnerBlacklistUnallowResult::Controller(controller_blacklist_unallow_result) => {
                                match controller_blacklist_unallow_result {
                                    ControllerBlacklistUnallowResult::Blacklist(access_control_blacklist_unallow_result) => {
                                        match access_control_blacklist_unallow_result {
                                            AccessControlBlacklistUnallowResult::Blacklist(blacklist_unallow_result) => {
                                                match blacklist_unallow_result {
                                                    BlacklistUnallowResult::Unallow(result) => {
                                                        match result {
                                                            true => Self::Ok,
                                                            false => Self::Err,
                                                        }
                                                    },
                                                }
                                            },
                                        }
                                    },
                                    ControllerBlacklistUnallowResult::Denied => {
                                        Self::OwnershipDenied
                                    },
                                }
                            },
                            OwnerBlacklistUnallowResult::Denied(authentication_result) => {
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

pub enum RegistryWhitelistUnallowResult {
    Ok,
    Err,
    OwnershipDenied,
    VerificationFailure
}

impl From<SingularRegistryWhitelistUnallowResult> for RegistryWhitelistUnallowResult {
    fn from(value: SingularRegistryWhitelistUnallowResult) -> Self {
        match value {
            SingularRegistryWhitelistUnallowResult::Reception(reception_whitelist_unallow_result) => {
                match reception_whitelist_unallow_result {
                    ReceptionWhitelistUnallowResult::Owner(owner_whitelist_unallow_result) => {
                        match owner_whitelist_unallow_result {
                            OwnerWhitelistUnallowResult::Controller(controller_whitelist_unallow_result) => {
                                match controller_whitelist_unallow_result {
                                    ControllerWhitelistUnallowResult::Whitelist(access_control_whitelist_unallow_result) => {
                                        match access_control_whitelist_unallow_result {
                                            AccessControlWhitelistUnallowResult::Whitelist(whitelist_unallow_result) => {
                                                match whitelist_unallow_result {
                                                    WhitelistUnallowResult::Unallow(result) => {
                                                        match result {
                                                            true => Self::Ok,
                                                            false => Self::Err,
                                                        }
                                                    },
                                                }
                                            },
                                        }
                                    },
                                    ControllerWhitelistUnallowResult::Denied => {
                                        Self::OwnershipDenied
                                    },
                                }
                            },
                            OwnerWhitelistUnallowResult::Denied(authentication_result) => {
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

pub enum RegistryCheckAccessResult {
    Err,
    NoCurrentAccess,
    ReservationConflict,
    VerificationFailure,
    NeedIdPassword,
    OwnershipDenied, // OwnershipDenied is invoked if Whitelist and Blacklist would deny and last resort using Ids doesnt work
    WhitelistDenied,
    BlacklistDenied,
    ContainsResource,
    MissingResource,
}

impl From<SingularRegistryCheckAccessResult> for RegistryCheckAccessResult {
    fn from(value: SingularRegistryCheckAccessResult) -> Self {
        match value {
            SingularRegistryCheckAccessResult::Reception(reception_check_access_result) => {
                match reception_check_access_result {
                    ReceptionCheckAccessResult::Host(host_check_access_result) => {
                        match host_check_access_result {
                            HostCheckAccessResult::Accesses(accesses_check_access_result) => {
                                match accesses_check_access_result {
                                    AccessesCheckAccessResult::Ok(result) => {
                                        assert_eq!(result, false);

                                        Self::Err
                                    },
                                    AccessesCheckAccessResult::NoCurrentAccess => {
                                        Self::NoCurrentAccess
                                    },
                                }
                            },
                            HostCheckAccessResult::ReservationConflict => {
                                Self::ReservationConflict
                            },
                        }
                    },
                    ReceptionCheckAccessResult::Denied(owner_check_access_result) => {
                        match owner_check_access_result {
                            OwnerCheckAccessResult::Controller(controller_check_access_result) => {
                                match controller_check_access_result {
                                    ControllerCheckAccessResult::Verification(resource_control_check_owner_result) => {
                                        match resource_control_check_owner_result {
                                            ResourceControlCheckOwnerResult::Verification(result) => {
                                                assert_eq!(result, false);

                                                Self::OwnershipDenied
                                            },
                                        }
                                    },
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Whitelist(whitelist_check_access_result) => {
                                                match whitelist_check_access_result {
                                                    WhitelistCheckAccessResult::Allowed(result) => {
                                                        assert_eq!(result, false);

                                                        Self::WhitelistDenied
                                                    },
                                                }
                                            },
                                            AccessControlCheckAccessResult::Blacklist(blacklist_check_access_result) => {
                                                match blacklist_check_access_result {
                                                    BlacklistCheckAccessResult::Verification(result) => {
                                                        assert_eq!(result, false);

                                                        Self::BlacklistDenied
                                                    },
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            OwnerCheckAccessResult::Denied(authentication_result) => {
                                match authentication_result {
                                    AuthenticationResult::Verification(result) => {
                                        assert_eq!(result, false);

                                        Self::VerificationFailure
                                    },
                                }
                            },
                            OwnerCheckAccessResult::NeedIdPassword => {
                                Self::NeedIdPassword
                            },
                        }
                    },
                }
            },
            SingularRegistryCheckAccessResult::AutomatedRegistry(result) => {
                match result {
                    true => Self::ContainsResource,
                    false => Self::MissingResource,
                }
            },
        }
    }
}

pub enum RegistryReleaseAccessResult {
    Ok,
    Err,
    NoCurrentAccess
}

impl From<SingularRegistryReleaseAccessResult> for RegistryReleaseAccessResult {
    fn from(value: SingularRegistryReleaseAccessResult) -> Self {
        match value {
            SingularRegistryReleaseAccessResult::Reception(reception_release_access_result) => {
                match reception_release_access_result {
                    ReceptionReleaseAccessResult::Host(host_release_access_result) => {
                        match host_release_access_result {
                            HostReleaseAccessResult::Accesses(accesses_release_result) => {
                                match accesses_release_result {
                                    AccessesReleaseResult::Split => {
                                        Self::Ok
                                    },
                                    AccessesReleaseResult::NoCurrentAccess => {
                                        Self::NoCurrentAccess
                                    },
                                }
                            },
                        }
                    },
                }
            },
            SingularRegistryReleaseAccessResult::AutomatedRegistry(manual_registry_release_result) => {
                match manual_registry_release_result {
                    ManualRegistryReleaseResult::Storage(result) => {
                        assert_eq!(result, false);

                        Self::Err
                    },
                }
            },
        }
    }
}

pub enum RegistryReservationResult {
    Ok,
    OkNew,
    ReservationConflict,
    AccessConflict,
    VerificationFailure,
    NeedIdPassword,
    OwnershipDenied,
    WhitelistDenied,
    BlacklistDenied
}

impl From<SingularRegistryReservationResult> for RegistryReservationResult {
    fn from(value: SingularRegistryReservationResult) -> Self {
        match value {
            SingularRegistryReservationResult::Reception(reception_reservation_result) => {
                match reception_reservation_result {
                    ReceptionReservationResult::Host(host_reservation_result) => {
                        match host_reservation_result {
                            HostReservationResult::Reservations(reservations_reserve_result) => {
                                match reservations_reserve_result {
                                    ReservationsReserveResult::FoundReserver => {
                                        Self::Ok
                                    },
                                    ReservationsReserveResult::FirstReservation => {
                                        Self::OkNew
                                    },
                                    ReservationsReserveResult::Reservations(reservations_check_access_result) => {
                                        match reservations_check_access_result {
                                            ReservationsCheckAccessResult::Ok(result) => {
                                                assert_eq!(result, false);

                                                Self::ReservationConflict
                                            },
                                        }
                                    },
                                }
                            },
                            HostReservationResult::AccessConflict(accesses_check_access_result) => {
                                assert!(!accesses_check_access_result.ok());
                                
                                Self::AccessConflict
                            }
                        }
                    },
                    ReceptionReservationResult::Denied(owner_authentication_result) => {
                        match owner_authentication_result {
                            OwnerCheckAccessResult::Controller(controller_check_access_result) => {
                                match controller_check_access_result {
                                    ControllerCheckAccessResult::Verification(resource_control_check_owner_result) => {
                                        match resource_control_check_owner_result {
                                            ResourceControlCheckOwnerResult::Verification(result) => {
                                                assert_eq!(result, false);

                                                Self::OwnershipDenied
                                            },
                                        }
                                    },
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Whitelist(whitelist_check_access_result) => {
                                                match whitelist_check_access_result {
                                                    WhitelistCheckAccessResult::Allowed(result) => {
                                                        assert_eq!(result, false);

                                                        Self::WhitelistDenied
                                                    },
                                                }
                                            },
                                            AccessControlCheckAccessResult::Blacklist(blacklist_check_access_result) => {
                                                match blacklist_check_access_result {
                                                    BlacklistCheckAccessResult::Verification(result) => {
                                                        assert_eq!(result, false);

                                                        Self::BlacklistDenied
                                                    },
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            OwnerCheckAccessResult::Denied(authentication_result) => {
                                match authentication_result {
                                    AuthenticationResult::Verification(result) => {
                                        assert_eq!(result, false);

                                        Self::VerificationFailure
                                    },
                                }
                            },
                            OwnerCheckAccessResult::NeedIdPassword => Self::NeedIdPassword
                            // OwnerAuthenticationResult::Authenticator(authentication_result) => {
                            //     match authentication_result {
                            //         AuthenticationResult::Verification(result) => {
                            //             assert_eq!(result, false);

                            //             Self::VerificationFailure
                            //         },
                            //     }
                            // },
                        }
                    },
                }
            },
        }
    }
}

pub enum RegistryUnreserveResult {
    Ok,
    NoReservation,
    NoReserver,
    VerificationFailure
}

impl From<SingularRegistryUnreserveResult> for RegistryUnreserveResult {
    fn from(value: SingularRegistryUnreserveResult) -> Self {
        match value {
            SingularRegistryUnreserveResult::Reception(reception_unreserve_result) => {
                match reception_unreserve_result {
                    ReceptionUnreserveResult::Host(host_unreserve_result) => {
                        match host_unreserve_result {
                            HostUnreserveResult::Reservations(reservations_unreserve_result) => {
                                match reservations_unreserve_result {
                                    ReservationsUnreserveResult::Accesses(accesses_release_result) => {
                                        match accesses_release_result {
                                            AccessesReleaseResult::Split => {
                                                Self::Ok
                                            },
                                            AccessesReleaseResult::NoCurrentAccess => {
                                                Self::NoReservation
                                            },
                                        }
                                    },
                                    ReservationsUnreserveResult::NoReserver => {
                                        Self::NoReserver
                                    },
                                }
                            },
                        }
                    },
                    ReceptionUnreserveResult::Denied(owner_authentication_result) => {
                        match owner_authentication_result {
                            OwnerAuthenticationResult::Authenticator(authentication_result) => {
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

pub enum RegistryDrainReservationsResult<T> {
    Drain(T),
    NoReserver,
    VerificationFailure
}

impl<T> From<SingularRegistryDrainReservationsResult<T>> for RegistryDrainReservationsResult<T> {
    fn from(value: SingularRegistryDrainReservationsResult<T>) -> Self {
        match value {
            SingularRegistryDrainReservationsResult::Reception(reception_drain_reservations_result) => {
                match reception_drain_reservations_result {
                    ReceptionDrainReservationsResult::Host(host_drain_reservations_result) => {
                        match host_drain_reservations_result {
                            HostDrainReservationsResult::Reservations(reservations_drain_reservations_result) => {
                                match reservations_drain_reservations_result {
                                    ReservationsDrainReservationsResult::Accesses(accesses_drain_result) => {
                                        match accesses_drain_result {
                                            AccessesDrainResult::Drain(result) => {
                                                Self::Drain(result)
                                            },
                                        }
                                    },
                                    ReservationsDrainReservationsResult::NoReserver => {
                                        Self::NoReserver
                                    },
                                }
                            },
                        }
                    },
                    ReceptionDrainReservationsResult::Denied(owner_authentication_result) => {
                        match owner_authentication_result {
                            OwnerAuthenticationResult::Authenticator(authentication_result) => {
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

pub enum RegistryAcquireAccessResult<AccessResult> {
    Found(AccessResult),
    NotFound,
    AccessConflict,
    ReservationConflict,
    VerificationFailure,
    OwnershipDenied,
    NeedIdPassword,
    WhitelistDenied,
    BlacklistDenied
}

impl<AccessResult> From<SingularRegistryAcquireAccessResult<AccessResult>> for RegistryAcquireAccessResult<AccessResult> {
    fn from(value: SingularRegistryAcquireAccessResult<AccessResult>) -> Self {
        match value {
            SingularRegistryAcquireAccessResult::AutomatedRegistry(manual_registry_access_result) => {
                match manual_registry_access_result {
                    ManualRegistryAccessResult::Found(result) => {
                        Self::Found(result)
                    },
                    ManualRegistryAccessResult::NotFound => {
                        Self::NotFound
                    },
                }
            },
            SingularRegistryAcquireAccessResult::Reception(reception_check_access_result) => {
                match reception_check_access_result {
                    ReceptionCheckAccessResult::Host(host_check_access_result) => {
                        match host_check_access_result {
                            HostCheckAccessResult::Accesses(accesses_check_access_result) => {
                                assert!(!accesses_check_access_result.ok());

                                Self::AccessConflict
                            },
                            HostCheckAccessResult::ReservationConflict => {
                                Self::ReservationConflict
                            },
                        }
                    },
                    ReceptionCheckAccessResult::Denied(owner_check_access_result) => {
                        match owner_check_access_result {
                            OwnerCheckAccessResult::Controller(controller_check_access_result) => {
                                match controller_check_access_result {
                                    ControllerCheckAccessResult::Verification(resource_control_check_owner_result) => {
                                        match resource_control_check_owner_result {
                                            ResourceControlCheckOwnerResult::Verification(result) => {
                                                assert_eq!(result, false);

                                                Self::OwnershipDenied
                                            },
                                        }
                                    },
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Whitelist(whitelist_check_access_result) => {
                                                match whitelist_check_access_result {
                                                    WhitelistCheckAccessResult::Allowed(result) => {
                                                        assert_eq!(result, false);

                                                        Self::WhitelistDenied
                                                    },
                                                }
                                            },
                                            AccessControlCheckAccessResult::Blacklist(blacklist_check_access_result) => {
                                                match blacklist_check_access_result {
                                                    BlacklistCheckAccessResult::Verification(result) => {
                                                        assert_eq!(result, false);

                                                        Self::BlacklistDenied
                                                    },
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            OwnerCheckAccessResult::Denied(authentication_result) => {
                                match authentication_result {
                                    AuthenticationResult::Verification(result) => {
                                        assert_eq!(result, false);

                                        Self::VerificationFailure
                                    },
                                }
                            },
                            OwnerCheckAccessResult::NeedIdPassword => {
                                Self::NeedIdPassword
                            },
                        }
                    },
                }
            },
        }
    }
}

pub enum RegistrySaferReplacementResult<ReplacementResult> {
    Found(ReplacementResult),
    NotFound,
    DeniedAccess,
    NoOp,

    AccessConflict,
    ReservationConflict,
    VerificationFailure,
    OwnershipDenied,
    NeedIdPassword,
    WhitelistDenied,
    BlacklistDenied,
}

impl<ReplacementResult> From<SingularRegistrySaferReplacementResult<ReplacementResult>> for RegistrySaferReplacementResult<ReplacementResult> {
    fn from(value: SingularRegistrySaferReplacementResult<ReplacementResult>) -> Self {
        match value {
            SingularRegistrySaferReplacementResult::AutomatedRegistry(manual_registry_replacement_result) => {
                match manual_registry_replacement_result {
                    ManualRegistryReplacementResult::Found(replacement_result) => {
                        Self::Found(replacement_result)
                    },
                    ManualRegistryReplacementResult::NotFound => {
                        Self::NotFound
                    },
                    ManualRegistryReplacementResult::DeniedAccess => {
                        Self::DeniedAccess
                    },
                    ManualRegistryReplacementResult::NoOp => {
                        Self::NoOp
                    },
                }
            },
            SingularRegistrySaferReplacementResult::Reception(reception_check_access_result) => {
                match reception_check_access_result {
                    ReceptionCheckAccessResult::Host(host_check_access_result) => {
                        match host_check_access_result {
                            HostCheckAccessResult::Accesses(accesses_check_access_result) => {
                                assert!(!accesses_check_access_result.ok());

                                Self::AccessConflict
                            },
                            HostCheckAccessResult::ReservationConflict => {
                                Self::ReservationConflict
                            },
                        }
                    },
                    ReceptionCheckAccessResult::Denied(owner_check_access_result) => {
                        match owner_check_access_result {
                            OwnerCheckAccessResult::Controller(controller_check_access_result) => {
                                match controller_check_access_result {
                                    ControllerCheckAccessResult::Verification(resource_control_check_owner_result) => {
                                        match resource_control_check_owner_result {
                                            ResourceControlCheckOwnerResult::Verification(result) => {
                                                assert_eq!(result, false);

                                                Self::OwnershipDenied
                                            },
                                        }
                                    }
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Whitelist(whitelist_check_access_result) => {
                                                match whitelist_check_access_result {
                                                    WhitelistCheckAccessResult::Allowed(result) => {
                                                        assert_eq!(result, false);

                                                        Self::WhitelistDenied
                                                    },
                                                }
                                            },
                                            AccessControlCheckAccessResult::Blacklist(blacklist_check_access_result) => {
                                                match blacklist_check_access_result {
                                                    BlacklistCheckAccessResult::Verification(result) => {
                                                        assert_eq!(result, false);

                                                        Self::BlacklistDenied
                                                    },
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            OwnerCheckAccessResult::Denied(authentication_result) => {
                                match authentication_result {
                                    AuthenticationResult::Verification(result) => {
                                        assert_eq!(result, false);

                                        Self::VerificationFailure
                                    },
                                }
                            },
                            OwnerCheckAccessResult::NeedIdPassword => {
                                Self::NeedIdPassword
                            },
                        }
                    },
                }
            }
        }
    }
}

pub enum RegistryContainsResourceResult {
    Some,
    None
}

impl From<SingularRegistryContainsResourceResult> for RegistryContainsResourceResult {
    fn from(value: SingularRegistryContainsResourceResult) -> Self {
        match value {
            SingularRegistryContainsResourceResult::AutomatedRegistry(result) => {
                match result {
                    true => Self::Some,
                    false => Self::None,
                }
            },
        }
    }
}