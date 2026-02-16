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
                            ReceptionPasswordGeneratorInput
                        },
                        reception_result::{
                            ReceptionAccessPermissionResult,
                            
                        },
                        host::{
                            Host,
                            host_input::{
                                HostAccessPermissionInput,
                                HostRecordAccessInput
                            },
                            host_result::{
                                HostAccessPermissionResult,
                                HostRecordAccessResult
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
                                    ReservationsUnreserveResult
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
                                OwnerAccessPermissionInput,
                                OwnerPasswordGeneratorInput
                            },
                            owner_result::{
                                OwnerAccessPermissionResult,
                                OwnerPasswordGeneratorResult
                            },
                            authenticator::{
                                Authenticator,
                                authenticator_input::{
                                    AuthenticateInput
                                },
                                authenticator_result::{
                                    AuthenticationResult
                                },
                                owner_storage::{
                                    OwnerStorage
                                },
                                ownership_storage::{
                                    OwnershipStorage
                                },
                            },
                            door::{
                                Door,
                                door_input::{
                                    DoorPermitsAccessInput,
                                    DoorGeneratePasswordInput
                                },
                                door_result::{
                                    DoorAccessPermissionResult,
                                    DoorGeneratePasswordResult
                                },
                                password_storage::{
                                    PasswordStorage
                                },
                                lock_storage::{
                                    LockStorage
                                },
                            },
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