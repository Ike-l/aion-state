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
                                HostAccessPermissionInput,
                                HostRecordAccessInput
                            },
                            host_result::{
                                HostAccessPermissionResult,
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
                                    ReservationsAccessPermissionInput, 
                                    ReserveInput,
                                    UnreserveInput
                                },
                                reservations_result::{
                                    ReservationsAccessPermissionResult,
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
                                    AccessPermission,
                                    RecordAccessResult,
                                    RemoveAccessResult
                                },
                                accesses_input::{
                                    PermitsAccessInput, 
                                    RecordAccessInput,
                                    RemoveAccessInput
                                }
                            }
                        },
                        owner::{
                            Owner,
                            owner_input::{
                                OwnerOwn,
                                OwnerRegister
                            },
                            owner_result::{
                                OwnerOwnResult,
                                OwnerRegisterResult
                            },
                            authenticator::{
                                Authenticator,
                                authenticator_input::{
                                    Authentication,
                                    AuthenticateRegister
                                },
                                authenticator_result::{
                                    AuthenticationResult,
                                    AuthenticateRegistrationResult
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
                                    ControllerAccess
                                },
                                controller_result::{
                                    ControllerOwnResult,
                                    ControllerReleaseResult,
                                    ControllerBlacklistAllowResult,
                                    ControllerWhitelistAllowResult,
                                    ControllerAccessResult
                                },
                                access_control::{
                                    AccessControl,
                                    access_control_input::{
                                        AccessControlAccess,
                                        AccessControlAllow,
                                        AccessControlRelease,
                                        AccessControlBlock
                                    },
                                    access_control_result::{
                                        AccessControlAccessResult,
                                        AccessControlBlacklistAllowResult,
                                        AccessControlWhitelistAllowResult,
                                        AccessControlReleaseResult,
                                        AccessControlBlacklistBlockResult,
                                        AccessControlWhitelistBlockResult,
                                    },
                                    blacklist::{
                                        Blacklist,
                                        blacklist_input::{
                                            BlacklistAccess,
                                            BlacklistAllow,
                                            BlacklistRelease,
                                            BlacklistBlock
                                        },
                                        blacklist_result::{
                                            BlacklistAccessResult,
                                            BlacklistAllowResult,
                                            BlacklistReleaseResult,
                                            BlacklistBlockResult
                                        },
                                        blacklist_storage::{
                                            BlacklistStorage
                                        }
                                    },
                                    whitelist::{
                                        Whitelist,
                                        whitelist_input::{
                                            WhitelistAccess,
                                            WhitelistAllow,
                                            WhitelistRelease,
                                            WhitelistBlock
                                        },
                                        whitelist_result::{
                                            WhitelistAccessResult,
                                            WhitelistAllowResult,
                                            WhitelistReleaseResult,
                                            WhitelistBlockResult
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
                                        ResourceControlVerification,
                                        ResourceControlRelease,
                                        ResourceControlOwn
                                    },
                                    resource_control_result::{
                                        ResourceControlVerificationResult,
                                        ResourceControlReleaseResult,
                                        ResourceControlOwnResult
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