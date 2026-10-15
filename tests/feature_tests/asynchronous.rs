use aion_state::prelude::{
    RegistryAcquireAccess, RegistryAcquireView, RegistryContainsResource, RegistryOwn,
    RegistryRegister, RegistryReleaseAccess, RegistryReleaseResource, RegistryReplacement,
    SynchronisedRegistryAcquireAccessError,
    SynchronisedRegistryAcquireViewError, SynchronisedRegistryCheckedReplacementResult,
    SynchronisedRegistryContainsResourceResult,
};

use crate::{block_on, create_registry, default::prelude::*};

#[test]
fn async_registry_can_store_acquire_and_release_a_resource() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let registry = create_registry(None);
    let resource_id = ResourceId::new_label("feature-async-access");
    assert!(block_on(registry.checked_replace_async(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    })).ok());
    assert!(block_on(registry.contains_resource_async(&RegistryContainsResource { resource_id: &resource_id })).ok());

    let access = Access::Shared(1);
    let value = block_on(registry.acquire_access_async::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: access.clone(),
        password: None,
    })).unwrap();
    drop(value);

    assert!(matches!(unsafe {
        block_on(registry.release_access_async(&RegistryReleaseAccess {
            resource_id: &resource_id,
            access: &access,
        }))
    }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
}

#[test]
fn async_registry_can_acquire_a_view() {
    use crate::default::storages::registry_storage::ResourceView;

    let registry = create_registry(None);
    let resource_ids = [
        ResourceId::new_label("feature-async-view-a"),
        ResourceId::new_label("feature-async-view-b"),
    ];
    for resource_id in &resource_ids {
        assert!(block_on(registry.checked_replace_async(RegistryReplacement {
            user_details: None,
            access: &Access::Replace,
            resource_id: resource_id.clone(),
            resource: Some(Resource::new("resource".to_string())),
            password: None,
        })).ok());
    }

    let view = block_on(registry.acquire_view_async::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: None,
        resource_details: resource_ids.iter().cloned().map(|resource_id| (resource_id, None)).collect(),
        access: Access::Shared(1),
    })).unwrap();
    assert!(matches!(view, AccessResult::Shared(ResourceView::Shared(_))));
    drop(view);
    for resource_id in &resource_ids {
        assert!(matches!(unsafe {
            block_on(registry.release_access_async(&RegistryReleaseAccess {
                resource_id,
                access: &Access::Shared(1),
            }))
        }, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
    }
}

#[test]
fn async_registry_reports_missing_zero_access_and_duplicate_view_errors() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let registry = create_registry(None);
    let missing = ResourceId::new_label("async-missing");
    assert!(matches!(
        block_on(registry.contains_resource_async(&RegistryContainsResource { resource_id: &missing })),
        SynchronisedRegistryContainsResourceResult::None
    ));
    let missing_result = block_on(registry.acquire_access_async::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: missing,
        access: Access::Shared(1),
        password: None,
    }));
    assert!(matches!(missing_result, Err(SynchronisedRegistryAcquireAccessError::NotFound)));

    let resource_id = ResourceId::new_label("async-zero-access");
    assert!(block_on(registry.checked_replace_async(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("resource".to_string())),
        password: None,
    })).ok());
    let zero_result = block_on(registry.acquire_access_async::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Shared(0),
        password: None,
    }));
    assert!(matches!(zero_result, Err(SynchronisedRegistryAcquireAccessError::TriedAcquiring)));

    let duplicate_result = block_on(registry.acquire_view_async::<AccessResult<
        crate::default::storages::registry_storage::ResourceView<'_, ResourceId, Resource>,
    >>(RegistryAcquireView {
        user_details: None,
        resource_details: vec![(resource_id.clone(), None), (resource_id, None)],
        access: Access::Shared(1),
    }));
    assert!(matches!(duplicate_result, Err(SynchronisedRegistryAcquireViewError::DuplicateIds(indices)) if indices == vec![1]));
}

#[test]
fn async_access_apis_preserve_credential_failures() {
    use crate::default::storages::registry_storage::{ResourceView, ResourceWrapper};

    let registry = create_registry(None);
    let owner = ReserverId::new("async-auth-owner");
    let owner_password = Password::new(91);
    let guest = ReserverId::new("async-auth-guest");
    let guest_password = Password::new(92);
    assert!(block_on(registry.register_async(RegistryRegister {
        id: owner.clone(),
        password: owner_password.clone(),
    })).ok());
    assert!(block_on(registry.register_async(RegistryRegister {
        id: guest.clone(),
        password: guest_password,
    })).ok());

    let resource_id = ResourceId::new_label("async-auth-protected");
    assert!(block_on(registry.own_async(RegistryOwn {
        id: owner.clone(),
        password: &owner_password,
        resource_id: resource_id.clone(),
    })).ok());
    assert!(block_on(registry.checked_replace_async(RegistryReplacement {
        user_details: Some((&owner, &owner_password)),
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("protected".to_string())),
        password: None,
    })).ok());

    let wrong_password = Password::new(0);
    let replacement_result = block_on(registry.checked_replace_async(RegistryReplacement {
        user_details: Some((&guest, &wrong_password)),
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("unauthorized".to_string())),
        password: None,
    }));
    assert!(matches!(replacement_result, SynchronisedRegistryCheckedReplacementResult::VerificationFailure));

    let access_result = block_on(registry.acquire_access_async::<AccessResult<ResourceWrapper<'_, Resource>>>(RegistryAcquireAccess {
        user_details: Some((&guest, &wrong_password)),
        resource_id: resource_id.clone(),
        access: Access::Shared(1),
        password: None,
    }));
    assert!(matches!(access_result, Err(SynchronisedRegistryAcquireAccessError::VerificationFailure)));

    let view_result = block_on(registry.acquire_view_async::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: Some((&guest, &wrong_password)),
        resource_details: vec![(resource_id.clone(), None)],
        access: Access::Shared(1),
    }));
    assert!(matches!(
        view_result,
        Err(SynchronisedRegistryAcquireViewError::Reception(items))
            if items.len() == 1
                && items.iter().all(|(index, error)|
                    *index == 0
                        && matches!(error, SynchronisedRegistryAcquireAccessError::VerificationFailure))
    ));

    let release_result = block_on(registry.release_resource_async(&RegistryReleaseResource {
        id: &owner,
        password: &wrong_password,
        resource_id: &resource_id,
    }));
    assert!(matches!(
        release_result,
        aion_state::prelude::SynchronisedRegistryReleaseResourceResult::VerificationFailure
    ));
}

#[test]
fn async_view_reports_conflicts_for_each_requested_resource() {
    use crate::default::storages::registry_storage::ResourceView;
    use aion_state::prelude::{RegistryDrainReservations, RegistryRegister, RegistryReservation};

    let registry = create_registry(None);
    let resource_ids = [
        ResourceId::new_label("async-view-conflict-a"),
        ResourceId::new_label("async-view-conflict-b"),
    ];
    for resource_id in &resource_ids {
        assert!(block_on(registry.checked_replace_async(RegistryReplacement {
            user_details: None,
            access: &Access::Replace,
            resource_id: resource_id.clone(),
            resource: Some(Resource::new("resource".to_string())),
            password: None,
        })).ok());
    }

    let id = ReserverId::new("async-view-reserver".to_string());
    let password = Password::new(81);
    assert!(block_on(registry.register_async(RegistryRegister { id: id.clone(), password: password.clone() })).ok());
    for resource_id in &resource_ids {
        assert!(matches!(block_on(registry.reserve_async(RegistryReservation {
            id: id.clone(),
            id_password: &password,
            resource_id: resource_id.clone(),
            access: Access::Unique,
            password: None,
        })), aion_state::prelude::SynchronisedRegistryReservationResult::Ok | aion_state::prelude::SynchronisedRegistryReservationResult::OkNew));
    }
    let replacement_result = block_on(registry.checked_replace_async(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_ids[0].clone(),
        resource: Some(Resource::new("replacement".to_string())),
        password: None,
    }));
    assert!(matches!(replacement_result, SynchronisedRegistryCheckedReplacementResult::ReservationConflict));

    let result = block_on(registry.acquire_view_async::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryAcquireView {
        user_details: None,
        resource_details: resource_ids.iter().cloned().map(|resource_id| (resource_id, None)).collect(),
        access: Access::Shared(1),
    }));
    assert!(matches!(
        result,
        Err(SynchronisedRegistryAcquireViewError::Reception(items))
            if items.len() == resource_ids.len()
                && items.iter().enumerate().all(|(index, (reported_index, error))|
                    index == *reported_index
                        && matches!(error, SynchronisedRegistryAcquireAccessError::ReservationConflict))
    ));
    assert!(matches!(block_on(registry.drain_reservations_async(&RegistryDrainReservations { id: &id, password: &password })), aion_state::prelude::SynchronisedRegistryDrainReservationsResult::Drain(_)));
}

#[cfg(feature = "releaser")]
#[test]
fn async_view_releaser_preserves_missing_resource_errors() {
    use aion_state::prelude::{AsyncViewReleaser, RegistryOwnedAcquireView, SynchronisedRegistryAcquireViewError};
    use crate::default::storages::registry_storage::ResourceView;
    use std::sync::Arc;

    let registry = Arc::new(create_registry(None));
    let missing = ResourceId::new_label("async-view-releaser-missing");
    let result = block_on(registry.async_acquire_released_view::<AccessResult<ResourceView<'_, ResourceId, Resource>>>(RegistryOwnedAcquireView {
        user_details: None,
        resource_details: vec![(missing, None)],
        access: Access::Shared(1),
    }));
    assert!(matches!(result, Err(SynchronisedRegistryAcquireViewError::NotFound)));
}
