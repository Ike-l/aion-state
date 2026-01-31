pub mod registry;
pub mod accessor;


pub mod prelude {
    pub const FUNCTION_LEVEL: tracing::Level = tracing::Level::DEBUG;
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
                reception::{
                    Reception,
                    reception_input::{
                        ReceptionAccessPermissionInput
                    },
                    reception_result::{
                        ReceptionAccessPermission,
                    },
                    host::{
                        Host,
                        reservations::{
                            Reservations,
                            reservations_input::{
                                ReservationMapAccessPermissionInput
                            },
                            // reservations_result::{

                            // },
                            reservation_map::{
                                ReservationMap
                            }
                        },
                        stored_accesses::{
                            StoredAccesses,
                            access_map::{
                                AccessMap,
                                // access_map_input::{

                                // },
                                access_map_result::{
                                    AccessPermission
                                }
                            }
                        }
                    },
                    owner::{
                        Owner,
                        authenticator::{
                            Authenticator
                        },
                        password_manager::{
                            PasswordManager
                        }
                    }
                },
                automated_registry::{
                    AutomatedRegistry,
                    manual_registry::{
                        ManualRegistry,
                        manual_registry_input::{
                            ManualRegistryAccessInput, 
                            ManualRegistryReplacementInput
                        },
                        manual_registry_result::{
                            ManualRegistryAccessResult, 
                            ManualRegistryReplacementResult
                        },
                        storage::{
                            Storage
                        }
                    }
                }
            }
        },
        accessor::{
            Accessor
        }
    };
}