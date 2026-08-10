use std::sync::Arc;

use aion_state::prelude::{RegistryContainsResource, RegistryIsOwned, RegistryOwn, RegistryRegister, RegistryReplacement};

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
                        // if resource is owned and not by this person then should be fail (change later when allow whitelist/blacklist)
                        if registry.is_owned(&RegistryIsOwned { resource_id: &resource_id }) {
                            if !registry.check_owner(&RegistryCheckOwner { id, resource_id }) {
                                assert!(!result.ok());
                            }
                        }

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
            },
            Command::Own { id, password, resource_id } => {
                let registered = registry.registered().contains(&id);
                let input = RegistryIsOwned {
                    resource_id: &resource_id
                };

                let is_owned = registry.is_owned(&input);
                let result = registry.own(RegistryOwn {
                    id,
                    password: &password,
                    resource_id: resource_id.clone(),
                });

                if !registered {
                    assert!(!result.ok());
                }

                if is_owned {
                    assert!(!result.ok());
                } else {
                    assert!(registry.is_owned(&input));
                    assert!(result.ok());
                }
            }
        }
    }
}