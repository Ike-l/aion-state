use crate::prelude::{AccessControlBlacklistAllowResult, AccessControlBlacklistUnallowResult, AccessControlCheckAccessResult, AccessControlReleaseAllResult, AccessControlWhitelistAllowResult, AccessControlWhitelistUnallowResult, AccessesCheckAccessResult, AccessesDrainResult, AccessesReleaseResult, AuthenticateRegistrationResult, AuthenticateUpdatePasswordResult, AuthenticationResult, BlacklistAllowResult, BlacklistCheckAccessResult, BlacklistReleaseAllResult, BlacklistUnallowResult, ControllerBlacklistAllowResult, ControllerBlacklistUnallowResult, ControllerCheckAccessResult, ControllerOwnResult, ControllerReleaseIdResult, ControllerReleaseResourceAllResult, ControllerReleaseResourceResult, ControllerWhitelistAllowResult, ControllerWhitelistUnallowResult, HostCheckAccessResult, HostDrainReservationsResult, HostReleaseAccessResult, HostReservationResult, HostUnreserveResult, ManualRegistryAccessError, ManualRegistryReplacementResult, OwnerAuthenticationResult, OwnerBlacklistAllowResult, OwnerBlacklistUnallowResult, OwnerCheckAccessResult, OwnerOwnResult, OwnerRegisterResult, OwnerReleaseResourceAllResult, OwnerReleaseResourceResult, OwnerUnregisterResult, OwnerUpdatePasswordResult, OwnerWhitelistAllowResult, OwnerWhitelistUnallowResult, ReceptionBlacklistAllowResult, ReceptionBlacklistUnallowResult, ReceptionCheckAccessResult, ReceptionDrainReservationsResult, ReceptionOwnResult, ReceptionRegisterResult, ReceptionReleaseAccessResult, ReceptionReleaseResourceAllResult, ReceptionReleaseResourceResult, ReceptionReservationResult, ReceptionUnregisterResult, ReceptionUnreserveResult, ReceptionUpdatePasswordResult, ReceptionWhitelistAllowResult, ReceptionWhitelistUnallowResult, ReservationsCheckAccessResult, ReservationsDrainReservationsResult, ReservationsReserveResult, ReservationsUnreserveResult, ResourceControlCheckOwnerResult, ResourceControlOwnResult, ResourceControlReleaseResult, SingularRegistryAcquireAccessError, SingularRegistryBlacklistAllowResult, SingularRegistryBlacklistUnallowResult, SingularRegistryCheckAccessResult, SingularRegistryContainsResourceResult, SingularRegistryDrainReservationsResult, SingularRegistryOwnResult, SingularRegistryRegisterResult, SingularRegistryReleaseAccessResult, SingularRegistryReleaseResourceAllResult, SingularRegistryReleaseResourceResult, SingularRegistryReservationResult, SingularRegistrySaferReplacementResult, SingularRegistryUnregisterResult, SingularRegistryUnreserveResult, SingularRegistryUpdatePasswordResult, SingularRegistryWhitelistAllowResult, SingularRegistryWhitelistUnallowResult, WhitelistAllowResult, WhitelistCheckAccessResult, WhitelistReleaseAllResult, WhitelistUnallowResult};

#[derive(Debug, thiserror::Error)]
pub enum RegistryRegisterResult {
    #[error("Credential Storage Register Ok")]
    Ok,
    #[error("Credential Storage Register Failure")]
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

#[derive(Debug, thiserror::Error)]
pub enum RegistryUnregisterResult {
    #[error("Credential Storage Unregister Failure")]
    AuthenticatorUnregisterFailure,
    #[error("When Releasing User Id: Whitelist Release All: {whitelist_result}, Blacklist Release All: {blacklist_result}")]
    Lists {
        whitelist_result: bool, 
        blacklist_result: bool
    },
    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl RegistryUnregisterResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Lists{ .. })
    }
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
                            OwnerUnregisterResult::AuthenticatorUnregisterFailure => {
                                Self::AuthenticatorUnregisterFailure
                            },
                            OwnerUnregisterResult::VerificationFailure => {
                                Self::VerificationFailure
                            }
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryUpdatePasswordResult {
    #[error("Credential Storage Update Password Ok")]
    Ok,
    #[error("Credential Storage Update Password Failure")]
    Err,
    #[error("Credential Storage Verification Failure")]
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
                            OwnerUpdatePasswordResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            },
        }
    }

}

#[derive(Debug, thiserror::Error)]
pub enum RegistryOwnResult {
    #[error("Control Storage Own Ok")]
    Ok,
    #[error("Control Storage Own Failure")]
    Err,
    #[error("Control Storage Ownership Conflict")]
    OwnershipConflict,
    #[error("Credential Storage Verification Failure")]
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
                            OwnerOwnResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryReleaseResourceResult {
    #[error("Control Storage Release Ok")]
    Ok,
    #[error("Control Storage Release Failure")]
    Err,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Credential Storage Verification Failure ")]
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
                            OwnerReleaseResourceResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryReleaseResourceAllResult {
    /// RegistryReleaseResourceResult::VerificationFailure is unreachable!()
    #[error("Controller Released with len: {}", .0.len())]
    All(Vec<RegistryReleaseResourceResult>),

    #[error("Credential Storage Verification Failure")]
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
                            OwnerReleaseResourceAllResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryBlacklistAllowResult<Password> {
    #[error("Blacklist Allow Ok with Password <hidden>")]
    Ok(Password),
    #[error("Blacklist Allow Failure")]
    Err,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Credential Storage Verification Failure")]
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
                            OwnerBlacklistAllowResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryWhitelistAllowResult {
    #[error("Whitelist Allow Ok")]
    Ok,
    #[error("Whitelist Allow Failure")]
    Err,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Credential Storage Verification Failure")]
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
                            OwnerWhitelistAllowResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryBlacklistUnallowResult {
    #[error("Blacklist Unallow Ok")]
    Ok,
    #[error("Blacklist Unallow Failure")]
    Err,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Credential Storage Verification Failure")]
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
                            OwnerBlacklistUnallowResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryWhitelistUnallowResult {
    #[error("Whitelist Unallow Ok")]
    Ok,
    #[error("Whitelist Unallow Failure")]
    Err,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Credential Storage Verification Failure")]
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
                            OwnerWhitelistUnallowResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryCheckAccessResult {
    #[error("Current Access does not Accept Incoming Access")]
    Err,
    #[error("Reservations Check Access Conflict Found")]
    ReservationConflict,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure,
    #[error("Control Storage Ownership Denied")] // OwnershipDenied is invoked if Whitelist and Blacklist would deny and last resort using Ids doesnt work
    OwnershipDenied,
    #[error("Whitelist Check Access Denied")]
    WhitelistDenied,
    #[error("Blacklist Check Access Denied")]
    BlacklistDenied,
    #[error("Registry Contains Resource")]
    ContainsResource,
    #[error("Registry Does not Contain Resource")]
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
                                        assert!(!result);

                                        Self::Err
                                    },
                                    AccessesCheckAccessResult::NoCurrentAccess => unreachable!("If no current access then it is Ok"),
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
                                                assert!(!result);

                                                Self::OwnershipDenied
                                            },
                                        }
                                    },
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Whitelist(whitelist_check_access_result) => {
                                                match whitelist_check_access_result {
                                                    WhitelistCheckAccessResult::Allowed(result) => {
                                                        assert!(!result);

                                                        Self::WhitelistDenied
                                                    },
                                                }
                                            },
                                            AccessControlCheckAccessResult::Blacklist(blacklist_check_access_result) => {
                                                match blacklist_check_access_result {
                                                    BlacklistCheckAccessResult::Verification(result) => {
                                                        assert!(!result);

                                                        Self::BlacklistDenied
                                                    },
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            OwnerCheckAccessResult::Denied => {
                                Self::VerificationFailure
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

#[derive(Debug, thiserror::Error)]
pub enum RegistryReleaseAccessResult {
    #[error("Split Access from a Current Access")]
    Ok,
    #[error("Registry Storage Release Failure")]
    Err,
    #[error("No Current Access to Release from")]
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
            SingularRegistryReleaseAccessResult::AutomatedRegistryReleaseFailure => {
                Self::Err
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryReservationResult {
    #[error("Reservation Made Ok is Composed with other Reservation(s)")]
    Ok,
    #[error("Reservation Made Ok is New")]
    OkNew,
    #[error("Reservation has Conflict")]
    ReservationConflict,
    #[error("Current Access Does not Accept Incoming Access")]
    AccessConflict,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Whitelist Access Denied")]
    WhitelistDenied,
    #[error("Blacklist Access Denied")]
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
                                                assert!(!result);

                                                Self::ReservationConflict
                                            },
                                        }
                                    },
                                }
                            },
                            HostReservationResult::AccessConflict => {
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
                                                assert!(!result);

                                                Self::OwnershipDenied
                                            },
                                        }
                                    },
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Whitelist(whitelist_check_access_result) => {
                                                match whitelist_check_access_result {
                                                    WhitelistCheckAccessResult::Allowed(result) => {
                                                        assert!(!result);

                                                        Self::WhitelistDenied
                                                    },
                                                }
                                            },
                                            AccessControlCheckAccessResult::Blacklist(blacklist_check_access_result) => {
                                                match blacklist_check_access_result {
                                                    BlacklistCheckAccessResult::Verification(result) => {
                                                        assert!(!result);

                                                        Self::BlacklistDenied
                                                    },
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            OwnerCheckAccessResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryUnreserveResult {
    #[error("Reservations Unreserve Ok")]
    Ok,
    #[error("No Reservation Found")]
    NoReservation,
    #[error("No Reserver")]
    NoReservationsMadeByReserver,
    #[error("Credential Storage Verification Failure")]
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
                                    ReservationsUnreserveResult::NoReservationsMadeByReserver => {
                                        Self::NoReservationsMadeByReserver
                                    },
                                }
                            },
                        }
                    },
                    ReceptionUnreserveResult::VerificationFailure => {
                        Self::VerificationFailure
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryDrainReservationsResult<Reservations> {
    #[error("Drained Reservations")]
    Drain(Reservations),
    #[error("No Reservervations Found")]
    NoReserver,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl<Reservations> From<SingularRegistryDrainReservationsResult<Reservations>> for RegistryDrainReservationsResult<Reservations> {
    fn from(value: SingularRegistryDrainReservationsResult<Reservations>) -> Self {
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
                                        assert!(!result);

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

#[derive(Debug, thiserror::Error)]
pub enum RegistryAcquireAccessError {
    #[error("Resource Not Found")]
    NotFound,
    #[error("Current Access Does not Accept Incoming Access")]
    AccessConflict,
    #[error("Reservation Conflict")]
    ReservationConflict,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Whitelist Access Denied")]
    WhitelistDenied,
    #[error("Blacklist Access Denied")]
    BlacklistDenied
}

impl From<SingularRegistryAcquireAccessError> for RegistryAcquireAccessError {
    fn from(value: SingularRegistryAcquireAccessError) -> Self {
        match value {
            SingularRegistryAcquireAccessError::AutomatedRegistry(manual_registry_access_result) => {
                match manual_registry_access_result {
                    ManualRegistryAccessError::NotFound => {
                        Self::NotFound
                    },
                }
            },
            SingularRegistryAcquireAccessError::Reception(reception_check_access_result) => {
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
                                                assert!(!result);

                                                Self::OwnershipDenied
                                            },
                                        }
                                    },
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Whitelist(whitelist_check_access_result) => {
                                                match whitelist_check_access_result {
                                                    WhitelistCheckAccessResult::Allowed(result) => {
                                                        assert!(!result);

                                                        Self::WhitelistDenied
                                                    },
                                                }
                                            },
                                            AccessControlCheckAccessResult::Blacklist(blacklist_check_access_result) => {
                                                match blacklist_check_access_result {
                                                    BlacklistCheckAccessResult::Verification(result) => {
                                                        assert!(!result);

                                                        Self::BlacklistDenied
                                                    },
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            OwnerCheckAccessResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistrySaferReplacementResult<ReplacementResult> {
    #[error("Replaced Resource and Returned Old")]
    Found(ReplacementResult),
    #[error("Inserted Resource")]
    NotFound,
    #[error("Given Access Denied Removal Or Insert")]
    DeniedAccess,
    #[error("Tried Inserting None")]
    NoOp,

    #[error("Current Access Does not Accept Incoming Access")]
    AccessConflict,
    #[error("Reservation Conflict")]
    ReservationConflict,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Whitelist Access Denied")]
    WhitelistDenied,
    #[error("Blacklist Access Denied")]
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
                                                assert!(!result);

                                                Self::OwnershipDenied
                                            },
                                        }
                                    }
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Whitelist(whitelist_check_access_result) => {
                                                match whitelist_check_access_result {
                                                    WhitelistCheckAccessResult::Allowed(result) => {
                                                        assert!(!result);

                                                        Self::WhitelistDenied
                                                    },
                                                }
                                            },
                                            AccessControlCheckAccessResult::Blacklist(blacklist_check_access_result) => {
                                                match blacklist_check_access_result {
                                                    BlacklistCheckAccessResult::Verification(result) => {
                                                        assert!(!result);

                                                        Self::BlacklistDenied
                                                    },
                                                }
                                            },
                                        }
                                    },
                                }
                            },
                            OwnerCheckAccessResult::Denied => {
                                Self::VerificationFailure
                            },
                        }
                    },
                }
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryContainsResourceResult {
    #[error("Registry Contains Resource")]
    Some,
    #[error("Registry Does not Contains Resource")]
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