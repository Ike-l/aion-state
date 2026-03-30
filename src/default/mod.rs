pub use crate::prelude::{Registry};

pub mod primitives;
pub mod storages;

pub mod prelude {
    pub use super::{
        primitives::{
            accesses::{
                access::{
                    Access,
                },
                access_result::{
                    AccessResult
                }
            },
            resources::{
                resource::{
                    Resource
                },
                resource_id::{
                    ResourceId
                },
                stored_resource::{
                    StoredResource
                }
            },
            users::{
                reserver_id::{
                    ReserverId
                },
                password::{
                    Password
                }
            }
        },
        storages::{
            registry_storage::{
                RegistryStorage
            },
            reservation_storage::{
                ReservationStorage
            },
            access_storage::{
                AccessStorage
            },
            credential_storage::{
                CredentialStorage
            },
            whitelist_storage::{
                WhitelistStorage
            },
            blacklist_storage::{
                BlacklistStorage
            },
            control_storage::{
                ControlStorage
            }
        }
    };
}