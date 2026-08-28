use std::sync::Arc;

use aion_state::prelude::{RegistryCheckOwner, RegistryCheckedReplacementResult, RegistryContainsResource, RegistryIsOwned, RegistryOwn, RegistryRegister, RegistryReplacement};
use tracing::{Level, event};

use crate::{TestRegistry, brute::command::Command, default::prelude::{Access, Password, ReserverId, Resource, ResourceId}};

pub struct PropertyManager {}

impl PropertyManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn test(&self, registry: &Arc<TestRegistry>, command: Command) {
        event!(Level::INFO, "Executing Command: {command:?}");
        match command {
            Command::CheckedReplacement{ user_details, access, resource_id, resource, password } => self.test_checked_replacement(registry, user_details, access, resource_id, resource, password),
            Command::Register { id, password } => self.test_register(registry, id, password),
            Command::Own { id, password, resource_id } => self.test_own(registry, id, password, resource_id),
        }
    }

    fn test_checked_replacement(
        &self,
        registry: &Arc<TestRegistry>,
        user_details: Option<(&ReserverId, &Password)>,
        access: Access,
        resource_id: ResourceId,
        resource: Option<Resource>,
        password: Option<Password>
    ) {
        let registry_contains_resource = RegistryContainsResource { resource_id: &resource_id };
        let contains = registry.contains_resource(&registry_contains_resource).ok();

        let resource_is_some = resource.is_some();

        let registered = user_details
            .is_some_and(|(id, _)| 
                registry.registered().contains(id)
            );

        let resources_before = registry.len();

        let registry_is_owned = RegistryIsOwned { resource_id: &resource_id };
        let owned = registry.is_owned(&registry_is_owned);

        let owned_by_user = user_details
            .as_ref()
            .is_some_and(|(id, _password)|
                registry.check_owner(&RegistryCheckOwner { id, resource_id: &resource_id }).ok() 
            );

        let input = RegistryReplacement {
            user_details,
            access: &access,
            resource_id: resource_id.clone(),
            resource,
            password: password.as_ref(),
        };

        let result = registry.checked_replace(input.clone());

        if !matches!(access, Access::Replace) {
            assert!(!result.ok());
            return;
        }

        self.assert_checked_replacement_result(
            registry, 
            result, 
            contains, 
            resource_is_some, 
            registered, 
            user_details.is_some(), 
            owned, 
            owned_by_user, 
            resources_before, 
            &registry_contains_resource
        );
    }

    fn assert_checked_replacement_result(
        &self,
        registry: &Arc<TestRegistry>,
        result: RegistryCheckedReplacementResult<Resource>,
        contains: bool,
        resource_is_some: bool,
        registered: bool,
        has_user_details: bool,
        owned: bool,
        owned_by_user: bool,
        resources_before: usize,
        registry_contains_resource: &RegistryContainsResource<'_, ResourceId> 
    ) {
        if owned {
            if !owned_by_user {
                assert!(!result.ok());
                return
            }

            self.assert_checked_replacement_mutation(
                registry,
                result,
                contains,
                resource_is_some,
                resources_before,
                registry_contains_resource
            );

            return
        }

        if !registered {
            if has_user_details {
                assert!(!result.ok());
                return
            }

            if !resource_is_some && contains {
                assert!(result.ok());
                return
            }

            if !resource_is_some {
                assert!(!result.ok());
                return
            }

            if !contains {
                assert!(result.ok());
                return
            }

            assert_eq!(resources_before, registry.len());
            assert!(registry.contains_resource(registry_contains_resource).ok());
            return
        }

        self.assert_checked_replacement_mutation(
            registry,
            result,
            contains,
            resource_is_some,
            resources_before,
            registry_contains_resource
        );
    }

    fn assert_checked_replacement_mutation(
        &self,
        registry: &Arc<TestRegistry>,
        result: RegistryCheckedReplacementResult<Resource>,
        contains: bool,
        resource_is_some: bool,
        resources_before: usize,
        registry_contains_resource: &RegistryContainsResource<'_, ResourceId> 
    ) {
        if !resource_is_some {
            if contains {
                assert_eq!(resources_before - 1, registry.len());
            } else {
                assert!(!result.ok());
            }

            return
        }

        if contains {
            assert_eq!(resources_before, registry.len());
        } else {
            // This can fail if len >= capacity and replacement reallocates
            assert_eq!(resources_before + 1, registry.len());
        }

        assert!(registry.contains_resource(registry_contains_resource).ok())
    }

    fn test_register(
        &self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password
    ) {
        let already_registered = registry.registered().contains(&id);
        
        let result = registry.register(RegistryRegister {
            id: id.clone(),
            password,
        });

        assert!(registry.registered().contains(&id));
        assert_eq!(!already_registered, result.ok());
    }

    fn test_own(
        &self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        resource_id: ResourceId
    ) {
        let registered = registry.registered().contains(&id);

        let input = RegistryIsOwned {
            resource_id: &resource_id
        };

        let already_owned = registry.is_owned(&input);

        let result = registry.own(RegistryOwn {
            id,
            password: &password,
            resource_id: resource_id.clone(),
        });

        match (registered, already_owned) {
            (false, _) => assert!(!result.ok()),
            (true, true) => assert!(!result.ok()),
            (true, false) => {
                assert!(result.ok());
                assert!(registry.is_owned(&input))
            }
        }
    }
}