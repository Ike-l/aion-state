pub mod registry;
pub mod accessor;

#[cfg(feature = "default-implementation")]
pub mod default;

pub mod prelude {
    pub const FUNCTION_LEVEL: tracing::Level = tracing::Level::DEBUG;

    #[cfg(not(all(test, feature = "loom")))]
    pub(crate) use std::sync;

    #[cfg(all(test, feature = "loom"))]
    pub(crate) use loom::sync;

    // #[cfg(feature = "tokio")]
    // pub(crate) use tokio::sync;

    macro_rules! trace_function {
        ($log:literal) => {
            let span = tracing::span!(crate::prelude::FUNCTION_LEVEL, $log);
            let _enter = span.enter();
        };
    }

    pub(crate) use trace_function;

    pub use super::{
        registry::{
            Registry,
            singular_registry::{
                SingularRegistry,
                singular_registry_input::{
                    SingularRegistryAccessInput
                },
                singular_registry_result::{
                    SingularRegistryAccessResult
                },
                coordinated_reception::{
                    CoordinatedReception,
                    reception::{
                        Reception,
                        reception_input::{
                            ReceptionAccessPermissionInput,
                            ReceptionPasswordGeneratorInput,
                            ReceptionReservationInput,
                            ReceptionUnreserveInput
                        },
                        reception_result::{
                        },
                        host::{
                            Host,
                            host_input::{
                                HostCheckAccess,
                                HostRecordAccess,
                                HostReleaseAccess,
                                HostReserve,
                                HostUnreserve
                            },
                            host_result::{
                                HostCheckAccessResult,
                                HostRecordAccessResult,
                                HostReservationResult,
                                HostUnreserveResult,
                                HostReleaseAccessResult,
                            },
                            reservations::{
                                Reservations,
                                reservation_storage::{
                                    ReservationStorage
                                },
                                reservations_input::{
                                    ReservationsCheckAccess, 
                                    ReservationsReserve,
                                    ReservationsUnreserve
                                },
                                reservations_result::{
                                    ReservationsCheckAccessResult,
                                    ReservationsReserveResult,
                                    ReservationsUnreserveResult,
                                },
                            },
                            accesses::{
                                Accesses,
                                access_storage::{
                                    AccessStorage
                                },
                                accesses_result::{
                                    AccessesCheckAccessResult,
                                    AccessesRecordAccessResult,
                                    AccessesReleaseResult
                                },
                                accesses_input::{
                                    AccessesCheckAccess, 
                                    AccessesRecordAccess,
                                    AccessesRelease
                                }
                            }
                        },
                        owner::{
                            Owner,
                            owner_input::{
                                OwnerOwn,
                                OwnerRegister,
                                OwnerRelease,
                                OwnerUpdatePassword,
                                OwnerUnregister,
                                OwnerAllow,
                                OwnerCheckAccess
                            },
                            owner_result::{
                                OwnerOwnResult,
                                OwnerRegisterResult,
                                OwnerReleaseResult,
                                OwnerUpdatePasswordResult,
                                OwnerUnregisterResult,
                                OwnerBlacklistAllowResult,
                                OwnerWhitelistAllowResult,
                                OwnerCheckAccessResult
                            },
                            authenticator::{
                                Authenticator,
                                authenticator_input::{
                                    Authentication,
                                    AuthenticateRegister,
                                    AuthenticateUpdatePassword,
                                    AuthenticateUnregister
                                },
                                authenticator_result::{
                                    AuthenticationResult,
                                    AuthenticateRegistrationResult,
                                    AuthenticateUpdatePasswordResult,
                                    AuthenticateUnregisterResult
                                },
                                credential_storage::{
                                    CredentialStorage
                                },
                            },
                            controller::{
                                Controller,
                                controller_input::{
                                    ControllerOwn,
                                    ControllerRelease,
                                    ControllerAllow,
                                    ControllerCheckAccess,
                                    ControllerReleaseId
                                },
                                controller_result::{
                                    ControllerOwnResult,
                                    ControllerReleaseResult,
                                    ControllerBlacklistAllowResult,
                                    ControllerWhitelistAllowResult,
                                    ControllerCheckAccessResult,
                                    ControllerReleaseIdResult
                                },
                                access_control::{
                                    AccessControl,
                                    access_control_input::{
                                        AccessControlCheckAccess,
                                        AccessControlAllow,
                                        AccessControlRelease,
                                        AccessControlBlock
                                    },
                                    access_control_result::{
                                        AccessControlCheckAccessResult,
                                        AccessControlBlacklistAllowResult,
                                        AccessControlWhitelistAllowResult,
                                        AccessControlReleaseResult,
                                        AccessControlBlacklistBlockResult,
                                        AccessControlWhitelistBlockResult,
                                        AccessControlReleaseAllResult,
                                    },
                                    blacklist::{
                                        Blacklist,
                                        blacklist_input::{
                                            BlacklistCheckAccess,
                                            BlacklistAllow,
                                            BlacklistRelease,
                                            BlacklistBlock
                                        },
                                        blacklist_result::{
                                            BlacklistCheckAccessResult,
                                            BlacklistAllowResult,
                                            BlacklistReleaseResult,
                                            BlacklistBlockResult,
                                            BlacklistReleaseAllResult
                                        },
                                        blacklist_storage::{
                                            BlacklistStorage
                                        }
                                    },
                                    whitelist::{
                                        Whitelist,
                                        whitelist_input::{
                                            WhitelistCheckAccess,
                                            WhitelistAllow,
                                            WhitelistRelease,
                                            WhitelistBlock
                                        },
                                        whitelist_result::{
                                            WhitelistCheckAccessResult,
                                            WhitelistAllowResult,
                                            WhitelistReleaseResult,
                                            WhitelistBlockResult,
                                            WhitelistReleaseAllResult
                                        },
                                        whitelist_storage::{
                                            WhitelistStorage
                                        }
                                    }
                                },
                                resource_control::{
                                    ResourceControl,
                                    control_storage::{
                                        ControlStorage
                                    },
                                    resource_control_input::{
                                        ResourceControlCheckResourceOwner,
                                        ResourceControlRelease,
                                        ResourceControlOwn,
                                        ResourceControlReleaseId
                                    },
                                    resource_control_result::{
                                        ResourceControlCheckResourceOwnerResult,
                                        ResourceControlReleaseResult,
                                        ResourceControlOwnResult,
                                        ResourceControlReleaseIdResult
                                    }
                                }
                            }
                        }
                    }
                },
                automated_registry::{
                    AutomatedRegistry,
                    manual_registry::{
                        ManualRegistry,
                        stable_address::{
                            StableAddress
                        },
                        manual_registry_input::{
                            ManualRegistryAccessInput, 
                            ManualRegistryReplacementInput
                        },
                        manual_registry_result::{
                            ManualRegistryAccessResult, 
                            ManualRegistryReplacementResult
                        },
                        registry_storage::{
                            RegistryStorage
                        },
                    }
                }
            }
        },
        accessor::{
            Accessor
        }
    };
}