#![allow(clippy::type_complexity)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::mut_from_ref)]
#![allow(clippy::single_match)]
#![allow(clippy::len_without_is_empty)]

pub mod synchronised_registry;
pub mod accessor;
#[cfg(feature = "releaser")]
pub mod releaser;
#[cfg(feature = "notifier")]
pub mod notifier;

pub mod prelude {
    pub(crate) const FUNCTION_LEVEL: tracing::Level = tracing::Level::TRACE;

    pub(crate) mod sync {
        #[allow(unused_imports)]
        pub use parking_lot::Mutex;
        pub use parking_lot::RwLock;
        #[allow(clippy::disallowed_types, unused_imports)]
        pub use std::sync::Arc;
    }

    // Features
    
    // Tokio
    // await synchronised registry
    // await coordinated reception

    // Notify 
    // notify on release

    macro_rules! trace_function {
        ($log:literal) => {
            let span = tracing::span!(crate::prelude::FUNCTION_LEVEL, $log);
            let _enter = span.enter();
        };
    }

    pub(crate) use trace_function;

    #[cfg(not(feature = "sync"))]
    pub use super::synchronised_registry::unsynchronised_registry::{
        UnsynchronisedRegistry as Registry,
        unsynchronised_registry_result::{
            UnsynchronisedRegistryRegisterResult as RegistryRegisterResult,
            UnsynchronisedRegistryUnregisterResult as RegistryUnregisterResult,
            UnsynchronisedRegistryUpdatePasswordResult as RegistryUpdatePasswordResult,
            UnsynchronisedRegistryOwnResult as RegistryOwnResult,
            UnsynchronisedRegistryReleaseResourceResult as RegistryReleaseResourceResult,
            UnsynchronisedRegistryReleaseResourceAllResult as RegistryReleaseResourceAllResult,
            UnsynchronisedRegistryBlacklistAllowResult as RegistryBlacklistAllowResult,
            UnsynchronisedRegistryWhitelistAllowResult as RegistryWhitelistAllowResult,
            UnsynchronisedRegistryBlacklistUnallowResult as RegistryBlacklistUnallowResult,
            UnsynchronisedRegistryWhitelistUnallowResult as RegistryWhitelistUnallowResult,
            UnsynchronisedRegistryCheckAccessResult as RegistryCheckAccessResult,
            UnsynchronisedRegistryReleaseAccessResult as RegistryReleaseAccessResult,
            UnsynchronisedRegistryReservationResult as RegistryReservationResult,
            UnsynchronisedRegistryUnreserveResult as RegistryUnreserveResult,
            UnsynchronisedRegistryDrainReservationsResult as RegistryDrainReservationsResult,
            UnsynchronisedRegistryAcquireAccessError as RegistryAcquireAccessError,
            UnsynchronisedRegistrySaferReplacementResult as RegistrySaferReplacementResult,
            UnsynchronisedRegistryContainsResourceResult as RegistryContainsResourceResult,
        },
    };
    
    #[cfg(feature = "sync")]
    pub use super::synchronised_registry::{
        SynchronisedRegistry as Registry,
        synchronised_registry_results::{
            SynchronisedRegistryRegisterResult as RegistryRegisterResult,
            SynchronisedRegistryAcquireAccessError as RegistryAcquireAccessError,
            SynchronisedRegistryBlacklistAllowResult as RegistryBlacklistAllowResult,
            SynchronisedRegistryBlacklistUnallowResult as RegistryBlacklistUnallowResult,
            SynchronisedRegistryCheckAccessResult as RegistryCheckAccessResult,
            SynchronisedRegistryContainsResourceResult as RegistryContainsResourceResult,
            SynchronisedRegistryDrainReservationsResult as RegistryDrainReservationsResult,
            SynchronisedRegistryOwnResult as RegistryOwnResult,
            SynchronisedRegistryReleaseAccessResult as RegistryReleaseAccessResult,
            SynchronisedRegistryReleaseResourceAllResult as RegistryReleaseResourceAllResult,
            SynchronisedRegistryReleaseResourceResult as RegistryReleaseResourceResult,
            SynchronisedRegistryReservationResult as RegistryReservationResult,
            SynchronisedRegistrySaferReplacementResult as RegistrySaferReplacementResult,
            SynchronisedRegistryUnregisterResult as RegistryUnregisterResult,
            SynchronisedRegistryUnreserveResult as RegistryUnreserveResult,
            SynchronisedRegistryUpdatePasswordResult as RegistryUpdatePasswordResult,
            SynchronisedRegistryWhitelistAllowResult as RegistryWhitelistAllowResult,
            SynchronisedRegistryWhitelistUnallowResult as RegistryWhitelistUnallowResult,
        },
    };

    #[cfg(feature = "releaser")]
    pub use super::{
        releaser::{
            Releaser,
            releasing_result::{
                ReleasingResult,
            }
        },
        synchronised_registry::unsynchronised_registry::registry_input::{
            RegistryReleasingAcquireAccess,
            RegistryReleasingReleaseAccess
        }
    };

    #[cfg(feature = "notifier")]
    pub use super::{
        notifier::{
            Notifier
        },
        synchronised_registry::{
            unsynchronised_registry::{
                registry_input::{
                    RegistryNotifiedAcquireAccess,
                },
                notify_queue::{
                    NotifyQueue
                }
            }
        }
    };

    pub use super::{
        synchronised_registry::{
            SynchronisedRegistry,
            synchronised_registry_results::{
                SynchronisedRegistryRegisterResult,
                SynchronisedRegistryAcquireAccessError,
                SynchronisedRegistryBlacklistAllowResult,
                SynchronisedRegistryBlacklistUnallowResult,
                SynchronisedRegistryCheckAccessResult,
                SynchronisedRegistryContainsResourceResult,
                SynchronisedRegistryDrainReservationsResult,
                SynchronisedRegistryOwnResult,
                SynchronisedRegistryReleaseAccessResult,
                SynchronisedRegistryReleaseResourceAllResult,
                SynchronisedRegistryReleaseResourceResult,
                SynchronisedRegistryReservationResult,
                SynchronisedRegistrySaferReplacementResult,
                SynchronisedRegistryUnregisterResult,
                SynchronisedRegistryUnreserveResult,
                SynchronisedRegistryUpdatePasswordResult,
                SynchronisedRegistryWhitelistAllowResult,
                SynchronisedRegistryWhitelistUnallowResult,
            },
            unsynchronised_registry::{
                UnsynchronisedRegistry,
                registry_input::{
                    RegistryRegister,
                    RegistryUnregister,
                    RegistryUpdatePassword,
                    RegistryOwn,
                    RegistryReleaseResource,
                    RegistryReleaseResourceAll,
                    RegistryAllow,
                    RegistryUnallow,
                    RegistryCheckAccess,
                    RegistryReleaseAccess,
                    RegistryReservation,
                    RegistryUnreserve,
                    RegistryDrainReservations,
                    RegistryAcquireAccess,
                    RegistrySaferReplacement,
                    RegistryContainsResource,
                },
                unsynchronised_registry_result::{
                    UnsynchronisedRegistryRegisterResult,
                    UnsynchronisedRegistryUnregisterResult,
                    UnsynchronisedRegistryUpdatePasswordResult,
                    UnsynchronisedRegistryOwnResult,
                    UnsynchronisedRegistryReleaseResourceResult,
                    UnsynchronisedRegistryReleaseResourceAllResult,
                    UnsynchronisedRegistryBlacklistAllowResult,
                    UnsynchronisedRegistryWhitelistAllowResult,
                    UnsynchronisedRegistryBlacklistUnallowResult,
                    UnsynchronisedRegistryWhitelistUnallowResult,
                    UnsynchronisedRegistryCheckAccessResult,
                    UnsynchronisedRegistryReleaseAccessResult,
                    UnsynchronisedRegistryReservationResult,
                    UnsynchronisedRegistryUnreserveResult,
                    UnsynchronisedRegistryDrainReservationsResult,
                    UnsynchronisedRegistryAcquireAccessError,
                    UnsynchronisedRegistrySaferReplacementResult,
                    UnsynchronisedRegistryContainsResourceResult,
                },
                coordinated_reception::{
                    CoordinatedReception,
                    reception::{
                        Reception,
                        reception_input::{
                            ReceptionRegister,
                            ReceptionUnregister,
                            ReceptionUpdatePassword,
                            ReceptionOwn,
                            ReceptionReleaseResource,
                            ReceptionAllow,
                            ReceptionUnallow,
                            ReceptionReleaseResourceAll,
                            ReceptionCheckAccess,
                            ReceptionReleaseAccess,
                            ReceptionRecordAccess,
                            ReceptionReservation,
                            ReceptionUnreserve,
                            ReceptionDrainReservations,
                            ReceptionGetAccess
                        },
                        reception_result::{
                            ReceptionRegisterResult,
                            ReceptionUnregisterResult,
                            ReceptionUpdatePasswordResult,
                            ReceptionOwnResult,
                            ReceptionReleaseResourceResult,
                            ReceptionWhitelistAllowResult,
                            ReceptionBlacklistAllowResult,
                            ReceptionWhitelistUnallowResult,
                            ReceptionBlacklistUnallowResult,
                            ReceptionReleaseResourceAllResult,
                            ReceptionCheckAccessResult,
                            ReceptionReleaseAccessResult,
                            ReceptionRecordAccessResult,
                            ReceptionReservationResult,
                            ReceptionUnreserveResult,
                            ReceptionDrainReservationsResult
                        },
                        host::{
                            Host,
                            host_input::{
                                HostCheckAccess,
                                HostRecordAccess,
                                HostReleaseAccess,
                                HostReservation,
                                HostUnreserve,
                                HostDrainReservations,
                                HostGetAccess,
                            },
                            host_result::{
                                HostCheckAccessResult,
                                HostRecordAccessResult,
                                HostReservationResult,
                                HostUnreserveResult,
                                HostReleaseAccessResult,
                                HostDrainReservationsResult,
                            },
                            reservations::{
                                Reservations,
                                reservation_storage::{
                                    ReservationStorage
                                },
                                reservations_input::{
                                    ReservationsCheckAccess, 
                                    ReservationsReservation,
                                    ReservationsUnreserve,
                                    ReservationsDrainReservations,
                                },
                                reservations_result::{
                                    ReservationsCheckAccessResult,
                                    ReservationsReserveResult,
                                    ReservationsUnreserveResult,
                                    ReservationsDrainReservationsResult
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
                                    AccessesReleaseResult,
                                    AccessesDrainResult
                                },
                                accesses_input::{
                                    AccessesCheckAccess, 
                                    AccessesRecordAccess,
                                    AccessesRelease,
                                    GetAccess,
                                }
                            }
                        },
                        owner::{
                            Owner,
                            owner_input::{
                                OwnerOwn,
                                OwnerRegister,
                                OwnerReleaseResource,
                                OwnerUpdatePassword,
                                OwnerUnregister,
                                OwnerAllow,
                                OwnerCheckAccess,
                                OwnerUnallow,
                                OwnerReleaseResourceAll,
                                OwnerAuthenticate
                            },
                            owner_result::{
                                OwnerOwnResult,
                                OwnerRegisterResult,
                                OwnerReleaseResourceResult,
                                OwnerUpdatePasswordResult,
                                OwnerUnregisterResult,
                                OwnerBlacklistAllowResult,
                                OwnerWhitelistAllowResult,
                                OwnerCheckAccessResult,
                                OwnerWhitelistUnallowResult,
                                OwnerBlacklistUnallowResult,
                                OwnerReleaseResourceAllResult,
                                OwnerAuthenticationResult
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
                                    ControllerReleaseResource,
                                    ControllerAllow,
                                    ControllerCheckAccess,
                                    ControllerReleaseId,
                                    ControllerCheckOwner,
                                    ControllerUnallow,
                                },
                                controller_result::{
                                    ControllerOwnResult,
                                    ControllerReleaseResourceResult,
                                    ControllerBlacklistAllowResult,
                                    ControllerWhitelistAllowResult,
                                    ControllerCheckAccessResult,
                                    ControllerReleaseIdResult,
                                    ControllerCheckOwnerResult,
                                    ControllerBlacklistUnallowResult,
                                    ControllerWhitelistUnallowResult,
                                    ControllerReleaseResourceAllResult
                                },
                                access_control::{
                                    AccessControl,
                                    access_control_input::{
                                        AccessControlCheckAccess,
                                        AccessControlAllow,
                                        AccessControlRelease,
                                        AccessControlUnallow
                                    },
                                    access_control_result::{
                                        AccessControlCheckAccessResult,
                                        AccessControlBlacklistAllowResult,
                                        AccessControlWhitelistAllowResult,
                                        AccessControlReleaseResult,
                                        AccessControlBlacklistUnallowResult,
                                        AccessControlWhitelistUnallowResult,
                                        AccessControlReleaseAllResult,
                                    },
                                    blacklist::{
                                        Blacklist,
                                        blacklist_input::{
                                            BlacklistCheckAccess,
                                            BlacklistAllow,
                                            BlacklistRelease,
                                            BlacklistUnallow
                                        },
                                        blacklist_result::{
                                            BlacklistCheckAccessResult,
                                            BlacklistAllowResult,
                                            BlacklistReleaseResult,
                                            BlacklistUnallowResult,
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
                                            WhitelistUnallow
                                        },
                                        whitelist_result::{
                                            WhitelistCheckAccessResult,
                                            WhitelistAllowResult,
                                            WhitelistReleaseResult,
                                            WhitelistUnallowResult,
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
                                        ResourceControlCheckOwner,
                                        ResourceControlRelease,
                                        ResourceControlOwn,
                                        ResourceControlReleaseId,
                                        ResourceIsOwned,
                                    },
                                    resource_control_result::{
                                        ResourceControlCheckOwnerResult,
                                        ResourceControlReleaseResult,
                                        ResourceControlOwnResult,
                                        ResourceControlReleaseIdResult,
                                        ResourceControlCheckOwnersResult,
                                        ResourceControlIsOwnedResult,
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
                        manual_registry_input::{
                            ManualRegistryAccessInput, 
                            ManualRegistryReplacementInput,
                        },
                        manual_registry_result::{
                            ManualRegistryAccessError, 
                            ManualRegistryReplacementResult,
                        },
                        registry_storage::{
                            RegistryStorage
                        },
                    }
                }
            }
        },
        accessor::{
            Accessor,
            AccessorResult,
            StoredValueTrait
        },
    };
}