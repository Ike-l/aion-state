use std::sync::Arc;

use aion_state::prelude::{RegistryAcquireAccess, RegistryContainsResource, RegistryRegister, RegistryReplacement};

use crate::{TestRegistry, brute::command::Command, default::prelude::Access};

pub struct PropertyManager {}

impl PropertyManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn test(&self, registry: &Arc<TestRegistry>, command: Command) {
        match command {
            Command::CheckedReplacement{ user_details, access, resource_id, resource, password } => {
                let resource_is_some = resource.is_some();
                let registry_contains_resource = RegistryContainsResource { resource_id: &resource_id };
                let resources = registry.len();
                let contains = registry.contains_resource(&registry_contains_resource).ok();

                let result = registry.checked_replace(RegistryReplacement {
                    user_details,
                    access: &access,
                    resource_id: resource_id.clone(),
                    resource,
                    password: password.as_ref(),
                });

                match access {
                    Access::Replace => {
                        if !resource_is_some {
                            if contains {
                                assert_eq!(resources - 1, registry.len());
                            } else {
                                assert!(!result.ok());
                            }
                        } else {
                            if !contains {
                                // can fail if len >= capacity (would reallocate)
                                // put test in later?
                                assert_eq!(resources + 1, registry.len(), "Result: {result}");
                            } else {
                                assert_eq!(resources, registry.len());
                            }
                            assert!(registry.contains_resource(&registry_contains_resource).ok());
                        }
                    },
                    _ => {
                        assert!(!result.ok())
                    },
                }
            },
            Command::Register { id, password } => {
                let already_registered = registry.registered().contains(&id);
                let result = registry.register(RegistryRegister {
                    id: id.clone(),
                    password,
                });

                assert!(registry.registered().contains(&id));
                assert_eq!(!already_registered, result.ok());
            }
        }
    }
}