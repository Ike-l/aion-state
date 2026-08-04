use aion_state::prelude::{RegistryAcquireAccess, SynchronisedRegistryAcquireAccessError, RegistryReplacement};

use crate::default::prelude::*;

use crate::create_registry;

#[test]
fn can_acquire_using_replace() {
    let registry = create_registry();

    let resource_id = ResourceId::new_type::<String>();
    let resource = Resource::new("resource".to_string());

    let result = registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(resource.clone()),
        password: None,
    });

    assert!(result.ok());

    let result = registry.acquire_access::<AccessResult<'_, Resource>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id,
        access: Access::Replace,
        password: None
    });

    assert!(result.is_err_and(|err| err == SynchronisedRegistryAcquireAccessError::TriedAcquiring));
}