use crate::prelude::{AccessControlBlacklistAllowResult, AccessControlBlacklistUnallowResult, AccessControlCheckAccessResult, AccessControlReleaseAllResult, AccessControlWhitelistAllowResult, AccessControlWhitelistUnallowResult, AccessesCheckAccessResult, AccessesDrainResult, AccessesReleaseResult, AuthenticateRegistrationResult, AuthenticateUpdatePasswordResult, AuthenticationResult, BlacklistAllowResult, BlacklistReleaseAllResult, BlacklistUnallowResult, ControllerBlacklistAllowResult, ControllerBlacklistUnallowResult, ControllerCheckAccessResult, ControllerOwnResult, ControllerReleaseIdResult, ControllerReleaseResourceAllResult, ControllerReleaseResourceResult, ControllerWhitelistAllowResult, ControllerWhitelistUnallowResult, HostCheckAccessResult, HostDrainReservationsResult, HostReleaseAccessResult, HostReservationResult, HostUnreserveResult, ManualRegistryAccessError, ManualRegistryCheckedReplacementResult, ManualRegistryReplacementResult, OwnerAuthenticationResult, OwnerBlacklistAllowResult, OwnerBlacklistUnallowResult, OwnerCheckAccessResult, OwnerOwnResult, OwnerRegisterResult, OwnerReleaseResourceAllResult, OwnerReleaseResourceResult, OwnerUnregisterResult, OwnerUpdatePasswordResult, OwnerWhitelistAllowResult, OwnerWhitelistUnallowResult, ReceptionBlacklistAllowResult, ReceptionBlacklistUnallowResult, ReceptionCheckAccessResult, ReceptionDrainReservationsResult, ReceptionOwnResult, ReceptionRegisterResult, ReceptionReleaseAccessResult, ReceptionReleaseResourceAllResult, ReceptionReleaseResourceResult, ReceptionReservationResult, ReceptionUnregisterResult, ReceptionUnreserveResult, ReceptionUpdatePasswordResult, ReceptionWhitelistAllowResult, ReceptionWhitelistUnallowResult, ReservationsCheckAccessResult, ReservationsDrainReservationsResult, ReservationsReserveResult, ReservationsUnreserveResult, ResourceControlOwnResult, ResourceControlReleaseResult, UnsynchronisedRegistryAcquireAccessError, UnsynchronisedRegistryBlacklistAllowResult, UnsynchronisedRegistryBlacklistUnallowResult, UnsynchronisedRegistryCheckAccessResult, UnsynchronisedRegistryCheckedReplacementResult, UnsynchronisedRegistryContainsResourceResult, UnsynchronisedRegistryDrainReservationsResult, UnsynchronisedRegistryOwnResult, UnsynchronisedRegistryReallocatingReplacementResult, UnsynchronisedRegistryRegisterResult, UnsynchronisedRegistryReleaseAccessResult, UnsynchronisedRegistryReleaseResourceAllResult, UnsynchronisedRegistryReleaseResourceResult, UnsynchronisedRegistryReservationResult, UnsynchronisedRegistryUnregisterResult, UnsynchronisedRegistryUnreserveResult, UnsynchronisedRegistryUpdatePasswordResult, UnsynchronisedRegistryWhitelistAllowResult, UnsynchronisedRegistryWhitelistUnallowResult, WhitelistAllowResult, WhitelistReleaseAllResult, WhitelistUnallowResult};

#[derive(Debug, thiserror::Error)]
pub enum SynchronisedRegistryRegisterResult {
    #[error("Credential Storage Register Ok")]
    Ok,
    #[error("Credential Storage Register Failure")]
    Err
}

impl SynchronisedRegistryRegisterResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Ok)
    }
}

impl From<UnsynchronisedRegistryRegisterResult> for SynchronisedRegistryRegisterResult {
    fn from(value: UnsynchronisedRegistryRegisterResult) -> Self {
        match value {
            UnsynchronisedRegistryRegisterResult::Reception(reception_register_result) => {
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
pub enum SynchronisedRegistryUnregisterResult {
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

impl SynchronisedRegistryUnregisterResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Lists{ .. })
    }
}

impl From<UnsynchronisedRegistryUnregisterResult> for SynchronisedRegistryUnregisterResult {
    fn from(value: UnsynchronisedRegistryUnregisterResult) -> Self {
        match value {
            UnsynchronisedRegistryUnregisterResult::Reception(reception_unregister_result) => {
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
pub enum SynchronisedRegistryUpdatePasswordResult {
    #[error("Credential Storage Update Password Ok")]
    Ok,
    #[error("Credential Storage Update Password Failure")]
    Err,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl SynchronisedRegistryUpdatePasswordResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Ok)
    }
}

impl From<UnsynchronisedRegistryUpdatePasswordResult> for SynchronisedRegistryUpdatePasswordResult {
    fn from(value: UnsynchronisedRegistryUpdatePasswordResult) -> Self {
        match value {
            UnsynchronisedRegistryUpdatePasswordResult::Reception(reception_update_password_result) => {
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
pub enum SynchronisedRegistryOwnResult {
    #[error("Control Storage Own Ok")]
    Ok,
    #[error("Control Storage Own Failure")]
    Err,
    #[error("Control Storage Ownership Conflict")]
    OwnershipConflict,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl SynchronisedRegistryOwnResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Ok)
    }
}

impl From<UnsynchronisedRegistryOwnResult> for SynchronisedRegistryOwnResult {
    fn from(value: UnsynchronisedRegistryOwnResult) -> Self {
        match value {
            UnsynchronisedRegistryOwnResult::Reception(reception_own_result) => {
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
pub enum SynchronisedRegistryReleaseResourceResult {
    #[error("Control Storage Release Ok")]
    Ok,
    #[error("Control Storage Release Failure")]
    Err,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Credential Storage Verification Failure ")]
    VerificationFailure
}

impl SynchronisedRegistryReleaseResourceResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Ok)
    }
}

impl From<UnsynchronisedRegistryReleaseResourceResult> for SynchronisedRegistryReleaseResourceResult {
    fn from(value: UnsynchronisedRegistryReleaseResourceResult) -> Self {
        match value {
            UnsynchronisedRegistryReleaseResourceResult::Reception(reception_release_resource_result) => {
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
pub enum SynchronisedRegistryReleaseResourceAllResult {
    /// RegistryReleaseResourceResult::VerificationFailure is unreachable!()
    #[error("Controller Released with len: {}", .0.len())]
    All(Vec<SynchronisedRegistryReleaseResourceResult>),

    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl From<UnsynchronisedRegistryReleaseResourceAllResult> for SynchronisedRegistryReleaseResourceAllResult {
    fn from(value: UnsynchronisedRegistryReleaseResourceAllResult) -> Self {
        match value {
            UnsynchronisedRegistryReleaseResourceAllResult::Reception(reception_release_resource_all_result) => {
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
                                                                true => SynchronisedRegistryReleaseResourceResult::Ok,
                                                                false => SynchronisedRegistryReleaseResourceResult::Err,
                                                            }
                                                        },
                                                    }
                                                },
                                                ControllerReleaseResourceResult::Denied => SynchronisedRegistryReleaseResourceResult::OwnershipDenied,
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
pub enum SynchronisedRegistryBlacklistAllowResult<Password> {
    #[error("Blacklist Allow Ok with Password <hidden>")]
    Ok(Password),
    #[error("Blacklist Allow Failure")]
    Err,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl<Password> SynchronisedRegistryBlacklistAllowResult<Password> {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }
}

impl<Password> From<UnsynchronisedRegistryBlacklistAllowResult<Password>> for SynchronisedRegistryBlacklistAllowResult<Password> {
    fn from(value: UnsynchronisedRegistryBlacklistAllowResult<Password>) -> Self {
        match value {
            UnsynchronisedRegistryBlacklistAllowResult::Reception(reception_blacklist_allow_result) => {
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
pub enum SynchronisedRegistryWhitelistAllowResult {
    #[error("Whitelist Allow Ok")]
    Ok,
    #[error("Whitelist Allow Failure")]
    Err,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl SynchronisedRegistryWhitelistAllowResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Ok => true,
            _ => false
        }
    }
}

impl From<UnsynchronisedRegistryWhitelistAllowResult> for SynchronisedRegistryWhitelistAllowResult {
    fn from(value: UnsynchronisedRegistryWhitelistAllowResult) -> Self {
        match value {
            UnsynchronisedRegistryWhitelistAllowResult::Reception(reception_whitelist_allow_result) => {
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
pub enum SynchronisedRegistryBlacklistUnallowResult {
    #[error("Blacklist Unallow Ok")]
    Ok,
    #[error("Blacklist Unallow Failure")]
    Err,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl From<UnsynchronisedRegistryBlacklistUnallowResult> for SynchronisedRegistryBlacklistUnallowResult {
    fn from(value: UnsynchronisedRegistryBlacklistUnallowResult) -> Self {
        match value {
            UnsynchronisedRegistryBlacklistUnallowResult::Reception(reception_blacklist_unallow_result) => {
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
pub enum SynchronisedRegistryWhitelistUnallowResult {
    #[error("Whitelist Unallow Ok")]
    Ok,
    #[error("Whitelist Unallow Failure")]
    Err,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl From<UnsynchronisedRegistryWhitelistUnallowResult> for SynchronisedRegistryWhitelistUnallowResult {
    fn from(value: UnsynchronisedRegistryWhitelistUnallowResult) -> Self {
        match value {
            UnsynchronisedRegistryWhitelistUnallowResult::Reception(reception_whitelist_unallow_result) => {
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
pub enum SynchronisedRegistryCheckAccessResult {
    #[error("Current Access does not Accept Incoming Access")]
    Err,
    #[error("Reservations Check Access Conflict Found")]
    ReservationConflict,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure,
    #[error("Whitelist & Blacklist Check Access Denied")]
    ListsDenied,
    #[error("Registry Contains Resource")]
    ContainsResource,
    #[error("Registry Does not Contain Resource")]
    MissingResource,
}

impl From<UnsynchronisedRegistryCheckAccessResult> for SynchronisedRegistryCheckAccessResult {
    fn from(value: UnsynchronisedRegistryCheckAccessResult) -> Self {
        match value {
            UnsynchronisedRegistryCheckAccessResult::Reception(reception_check_access_result) => {
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
                                    ControllerCheckAccessResult::IsOwner => unreachable!(),
                                    ControllerCheckAccessResult::NotOwned => unreachable!(),
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Lists { whitelist, blacklist } => {
                                                assert!(!whitelist.ok() && blacklist.is_some_and(|blacklist_result| blacklist_result.ok()));

                                                Self::ListsDenied
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
            UnsynchronisedRegistryCheckAccessResult::AutomatedRegistry(result) => {
                match result {
                    true => Self::ContainsResource,
                    false => Self::MissingResource,
                }
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SynchronisedRegistryReleaseAccessResult {
    #[error("Split Access from a Current Access")]
    Ok,
    #[error("No Current Access to Release from")]
    NoCurrentAccess
}

impl From<UnsynchronisedRegistryReleaseAccessResult> for SynchronisedRegistryReleaseAccessResult {
    fn from(value: UnsynchronisedRegistryReleaseAccessResult) -> Self {
        match value {
            UnsynchronisedRegistryReleaseAccessResult::Reception(reception_release_access_result) => {
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
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SynchronisedRegistryReservationResult {
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
    #[error("Whitelist & Blacklist Check Access Denied")]
    ListsDenied
}

impl From<UnsynchronisedRegistryReservationResult> for SynchronisedRegistryReservationResult {
    fn from(value: UnsynchronisedRegistryReservationResult) -> Self {
        match value {
            UnsynchronisedRegistryReservationResult::Reception(reception_reservation_result) => {
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
                                    ControllerCheckAccessResult::IsOwner => unreachable!(),
                                    ControllerCheckAccessResult::NotOwned => unreachable!(),
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Lists { whitelist, blacklist } => {
                                                assert!(!whitelist.ok() && blacklist.is_some_and(|blacklist_result| blacklist_result.ok()));

                                                Self::ListsDenied
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
pub enum SynchronisedRegistryUnreserveResult {
    #[error("Reservations Unreserve Ok")]
    Ok,
    #[error("No Reservation Found")]
    NoReservation,
    #[error("No Reserver")]
    NoReservationsMadeByReserver,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl From<UnsynchronisedRegistryUnreserveResult> for SynchronisedRegistryUnreserveResult {
    fn from(value: UnsynchronisedRegistryUnreserveResult) -> Self {
        match value {
            UnsynchronisedRegistryUnreserveResult::Reception(reception_unreserve_result) => {
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
pub enum SynchronisedRegistryDrainReservationsResult<Reservations> {
    #[error("Drained Reservations")]
    Drain(Reservations),
    #[error("No Reservervations Found")]
    NoReserver,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure
}

impl<Reservations> From<UnsynchronisedRegistryDrainReservationsResult<Reservations>> for SynchronisedRegistryDrainReservationsResult<Reservations> {
    fn from(value: UnsynchronisedRegistryDrainReservationsResult<Reservations>) -> Self {
        match value {
            UnsynchronisedRegistryDrainReservationsResult::Reception(reception_drain_reservations_result) => {
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

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum SynchronisedRegistryAcquireAccessError {
    #[error("Resource Not Found")]
    NotFound,
    #[error("Current Access Does not Accept Incoming Access")]
    AccessConflict,
    #[error("Reservation Conflict")]
    ReservationConflict,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure,
    #[error("Whitelist & Blacklist Check Access Denied")]
    ListsDenied,
    #[error("Access Denied Acquiring")]
    TriedAcquiring
}

impl From<UnsynchronisedRegistryAcquireAccessError> for SynchronisedRegistryAcquireAccessError {
    fn from(value: UnsynchronisedRegistryAcquireAccessError) -> Self {
        match value {
            UnsynchronisedRegistryAcquireAccessError::AutomatedRegistry(manual_registry_access_result) => {
                match manual_registry_access_result {
                    ManualRegistryAccessError::NotFound => {
                        Self::NotFound
                    },
                    ManualRegistryAccessError::TriedAcquiring => {
                        Self::TriedAcquiring
                    }
                }
            },
            UnsynchronisedRegistryAcquireAccessError::Reception(reception_check_access_result) => {
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
                                    ControllerCheckAccessResult::IsOwner => unreachable!(),
                                    ControllerCheckAccessResult::NotOwned => unreachable!(),
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Lists { whitelist, blacklist } => {
                                                assert!(!whitelist.ok() && blacklist.is_none_or(|blacklist_result| !blacklist_result.ok()));

                                                Self::ListsDenied
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
pub enum SynchronisedRegistryReallocatingReplacementResult<ReplacementResult> {
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
    #[error("Whitelist & Blacklist Check Access Denied")]
    ListsDenied
}

impl<T> SynchronisedRegistryReallocatingReplacementResult<T> {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Found(_)) || matches!(self, Self::NotFound)
    }
}

impl<ReplacementResult> From<UnsynchronisedRegistryReallocatingReplacementResult<ReplacementResult>> for SynchronisedRegistryReallocatingReplacementResult<ReplacementResult> {
    fn from(value: UnsynchronisedRegistryReallocatingReplacementResult<ReplacementResult>) -> Self {
        match value {
            UnsynchronisedRegistryReallocatingReplacementResult::AutomatedRegistry(manual_registry_replacement_result) => {
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
                    }
                }
            },
            UnsynchronisedRegistryReallocatingReplacementResult::Reception(reception_check_access_result) => {
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
                                    ControllerCheckAccessResult::IsOwner => unreachable!(),
                                    ControllerCheckAccessResult::NotOwned => unreachable!(),
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Lists { whitelist, blacklist } => {
                                                assert!(!whitelist.ok() && blacklist.is_some_and(|blacklist_result| blacklist_result.ok()));

                                                Self::ListsDenied
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
pub enum SynchronisedRegistryCheckedReplacementResult<ReplacementResult> {
    #[error("Replaced Resource and Returned Old")]
    Found(ReplacementResult),
    #[error("Inserted Resource")]
    NotFound,
    #[error("Given Access Denied Removal Or Insert")]
    DeniedAccess,
    #[error("Tried Inserting None and Removing None")]
    NoOp,
    #[error("Tried Removing Resource Which Would Reallocate")]
    RemovalReallocates,
    #[error("Tried Inserting Resource Which Would Reallocate")]
    InsertingReallocates,

    #[error("Current Access Does not Accept Incoming Access")]
    AccessConflict,
    #[error("Reservation Conflict")]
    ReservationConflict,
    #[error("Credential Storage Verification Failure")]
    VerificationFailure,
    #[error("Control Storage Ownership Denied")]
    OwnershipDenied,
    #[error("Whitelist & Blacklist Check Access Denied")]
    ListsDenied
}

impl<T> SynchronisedRegistryCheckedReplacementResult<T> {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Found(_)) || matches!(self, Self::NotFound)
    }
}

impl<ReplacementResult> From<UnsynchronisedRegistryCheckedReplacementResult<ReplacementResult>> for SynchronisedRegistryCheckedReplacementResult<ReplacementResult> {
    fn from(value: UnsynchronisedRegistryCheckedReplacementResult<ReplacementResult>) -> Self {
        match value {
            UnsynchronisedRegistryCheckedReplacementResult::AutomatedRegistry(manual_registry_checked_replacement_result) => {
                match manual_registry_checked_replacement_result {
                    ManualRegistryCheckedReplacementResult::ReplacementResult(manual_registry_replacement_result) => {
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
                            }
                        }
                    },
                    ManualRegistryCheckedReplacementResult::RemovalReallocates => {
                        Self::RemovalReallocates
                    },
                    ManualRegistryCheckedReplacementResult::InsertingReallocates => {
                        Self::InsertingReallocates
                    },
                }
            },
            UnsynchronisedRegistryCheckedReplacementResult::Reception(reception_check_access_result) => {
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
                                    ControllerCheckAccessResult::IsOwner => unreachable!(),
                                    ControllerCheckAccessResult::NotOwned => unreachable!(),
                                    ControllerCheckAccessResult::AccessControl(access_control_check_access_result) => {
                                        match access_control_check_access_result {
                                            AccessControlCheckAccessResult::Lists { whitelist, blacklist } => {
                                                assert!(!whitelist.ok() || blacklist.is_some_and(|blacklist_result| !blacklist_result.ok()));

                                                Self::ListsDenied
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
pub enum SynchronisedRegistryContainsResourceResult {
    #[error("Registry Contains Resource")]
    Some,
    #[error("Registry Does not Contains Resource")]
    None
}

impl SynchronisedRegistryContainsResourceResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Some)
    }
}

impl From<UnsynchronisedRegistryContainsResourceResult> for SynchronisedRegistryContainsResourceResult {
    fn from(value: UnsynchronisedRegistryContainsResourceResult) -> Self {
        match value {
            UnsynchronisedRegistryContainsResourceResult::AutomatedRegistry(result) => {
                match result {
                    true => Self::Some,
                    false => Self::None,
                }
            },
        }
    }
}