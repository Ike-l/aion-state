#[cfg(any(feature = "releaser", feature = "notifier", feature = "async", feature = "stable_deref"))]
use std::sync::Arc;

#[cfg(any(feature = "releaser", feature = "notifier", feature = "async"))]
use aion_state::prelude::ReceptionGetAccess;
#[cfg(feature = "stable_deref")]
use aion_state::prelude::RegistryContainsResource;
#[cfg(feature = "async")]
use aion_state::prelude::RegistryIsOwned;
#[cfg(any(feature = "releaser", feature = "notifier", feature = "async", feature = "stable_deref"))]
use super::{command::Command, property_manager::PropertyManager};
#[cfg(any(feature = "releaser", feature = "notifier", feature = "async", feature = "stable_deref"))]
use crate::{create_brute_registry, default::prelude::*};

#[cfg(feature = "releaser")]
#[test]
fn brute_releaser_command_releases_access_and_view_guards() {
    let registry = Arc::new(create_brute_registry(None));
    let mut manager = PropertyManager::new();

    manager.test(&registry, Command::Releaser);

    assert_eq!(registry.keys().len(), registry.len());
    for resource_id in registry.keys() {
        assert_eq!(Some(Access::Shared(0)), registry.get_access(&ReceptionGetAccess { access_id: &resource_id }));
    }
}

#[cfg(feature = "notifier")]
#[test]
fn brute_notifier_command_registers_and_wakes_a_waiter() {
    let registry = Arc::new(create_brute_registry(None));
    let mut manager = PropertyManager::new();

    manager.test(&registry, Command::Notifier);

    assert_eq!(1, registry.len());
    let resource_id = registry.keys().pop().unwrap();
    assert_eq!(Some(Access::Shared(0)), registry.get_access(&ReceptionGetAccess { access_id: &resource_id }));
}

#[cfg(feature = "async")]
#[test]
fn brute_async_command_runs_async_registry_operations() {
    let registry = Arc::new(create_brute_registry(None));
    let mut manager = PropertyManager::new();

    manager.test(&registry, Command::Async);

    assert_eq!(2, registry.len());
    assert!(registry.registered().is_empty());
    for resource_id in registry.keys() {
        assert!(!registry.is_owned(&RegistryIsOwned { resource_id: &resource_id }));
        assert_eq!(Some(Access::Shared(0)), registry.get_access(&ReceptionGetAccess { access_id: &resource_id }));
    }
}

#[cfg(feature = "stable_deref")]
#[test]
fn brute_stable_deref_command_uses_reallocating_replacement() {
    let registry = Arc::new(create_brute_registry(None));
    let mut manager = PropertyManager::new();
    let resource_id = ResourceId::new_label("brute-unit-stable-deref");

    manager.test(&registry, Command::ReallocatingReplacement {
        user_details: None,
        credentials_match: true,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("stable-value".to_string())),
        password: None,
    });

    assert!(registry.contains_resource(&RegistryContainsResource { resource_id: &resource_id }).ok());
    assert_eq!(1, registry.len());
}
