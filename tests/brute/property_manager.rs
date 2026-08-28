use std::sync::Arc;

use aion_state::prelude::{RegistryCheckOwner, RegistryContainsResource, RegistryIsOwned, RegistryOwn, RegistryRegister, RegistryReplacement};
use tracing::{Level, event};

use crate::{TestRegistry, brute::command::Command, default::prelude::Access};

pub struct PropertyManager {}

impl PropertyManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn test(&self, registry: &Arc<TestRegistry>, command: Command) {
        event!(Level::INFO, "Executing Command: {command:?}");
        match command {
            Command::CheckedReplacement{ user_details, access, resource_id, resource, password } => {
                let resource_is_some = resource.is_some();
                let registry_contains_resource = RegistryContainsResource { resource_id: &resource_id };
                let resources = registry.len();
                let contains = registry.contains_resource(&registry_contains_resource).ok();

                let registered = user_details.is_some_and(|(id, _)| registry.registered().contains(id));

                let input = RegistryReplacement {
                    user_details,
                    access: &access,
                    resource_id: resource_id.clone(),
                    resource,
                    password: password.as_ref(),
                };
                let before_string = serde_json::to_string(registry.as_ref()).unwrap();
                let result = registry.checked_replace(input.clone());

                match access {
                    Access::Replace => {
                        // (change later when allow whitelist/blacklist)
                        let is_owned = registry.is_owned(&RegistryIsOwned { resource_id: &resource_id });
                        let owned_by_me = user_details.is_some_and(|(id, _)| registry.check_owner(&RegistryCheckOwner { id, resource_id: &resource_id }).ok());

                        if is_owned {
                            if !owned_by_me {
                                assert!(!result.ok());
                            } else {
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
                            }
                        } else {
                            if !resource_is_some {
                                if !registered {   
                                    if user_details.is_none() {
                                        if !contains {
                                            assert!(!result.ok());
                                        } else {
                                            assert!(result.ok());
                                        }
                                    } else {
                                        assert!(!result.ok());
                                    }
                                } else {
                                    if contains {
                                        assert_eq!(resources - 1, registry.len());
                                    } else {
                                        assert!(!result.ok());
                                    }
                                }
                            } else {
                                if !contains {
                                    if !registered {
                                        if user_details.is_none() {
                                            assert!(result.ok());
                                        } else {
                                            assert!(!result.ok());
                                        }
                                    } else {
                                        // can fail if len >= capacity (would reallocate)
                                        // put test in later?
                                        assert_eq!(resources + 1, registry.len(), "Result: {result}");
                                    }
                                } else {
                                    assert_eq!(resources, registry.len());
                                }

                                if !contains && !registered {
                                    // todo!();
                                } else {
                                    assert!(registry.contains_resource(&registry_contains_resource).ok());
                                }
                            }
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
                } else {
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
}