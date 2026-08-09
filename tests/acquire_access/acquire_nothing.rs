use aion_state::prelude::{RegistryAcquireAccess, SynchronisedRegistryAcquireAccessError, RegistryReplacement};

use crate::default::prelude::*;

use crate::create_registry;

#[test]
fn can_acquire_nothing() {
    let registry = create_registry(None);

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

    let wrong_resource_id = ResourceId::new_type::<i32>();

    let result = registry.acquire_access::<AccessResult<'_, Resource>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: wrong_resource_id,
        access: Access::Shared(1),
        password: None
    });

    assert!(result.is_err_and(|err| err == SynchronisedRegistryAcquireAccessError::NotFound));
}