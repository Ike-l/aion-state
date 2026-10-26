#[allow(dead_code, unused_imports)]
#[path = "default/mod.rs"]
mod default;

use aion_state::{
    prelude::{
        RegistryAcquireAccess, RegistryAcquireView, RegistryAllow, RegistryCheckAccess,
        RegistryContainsResource, RegistryDrainReservations, RegistryIsOwned, RegistryOwn,
        RegistryRegister, RegistryReleaseAccess, RegistryReleaseResource,
        RegistryReleaseResourceAll, RegistryReplacement, RegistryReservation,
        RegistryUnallow, RegistryUnregister, RegistryUnreserve, RegistryUpdatePassword,
        ReceptionGetAccess,
        ControllerCheckAccessResult, HostCheckAccessResult,
        OwnerCheckAccessResult, ReceptionCheckAccessResult,
        SynchronisedRegistryBlacklistAllowResult, SynchronisedRegistryBlacklistUnallowResult,
        SynchronisedRegistryCheckAccessResult,
        SynchronisedRegistryCheckedReplacementResult, SynchronisedRegistryDrainReservationsResult,
        SynchronisedRegistryOwnResult, SynchronisedRegistryRegisterResult,
        SynchronisedRegistryReleaseAccessResult, SynchronisedRegistryReleaseResourceAllResult,
        SynchronisedRegistryReleaseResourceResult, SynchronisedRegistryReservationResult,
        SynchronisedRegistryWhitelistUnallowResult, SynchronisedRegistryUnregisterResult,
        SynchronisedRegistryUnreserveResult, SynchronisedRegistryUpdatePasswordResult,
    },
    synchronised_registry::unsynchronised_registry::{
        UnsynchronisedRegistry, unsynchronised_registry_result::{
            UnsynchronisedRegistryCheckAccessResult,
            UnsynchronisedRegistryContainsResourceResult,
            UnsynchronisedRegistryCheckedReplacementResult,
        },
    },
};
use default::prelude::{Access, AccessResult, Password, ReserverId, Resource, ResourceId};
#[cfg(not(feature = "stable_deref"))]
use default::prelude::StoredResource;
#[cfg(feature = "releaser")]
use std::sync::Arc;
#[cfg(feature = "releaser")]
use aion_state::prelude::{
    RegistryOwnedAcquireAccess, RegistryOwnedAcquireView, Releaser,
    UnsynchronisedRegistryAcquireAccessError, UnsynchronisedRegistryAcquireViewError,
    ViewReleaser,
};
#[cfg(feature = "releaser")]
use default::storages::registry_storage::{ResourceView, ResourceWrapper};

#[cfg(feature = "stable_deref")]
type TestStoredResource = Box<Resource>;
#[cfg(not(feature = "stable_deref"))]
type TestStoredResource = StoredResource;

type TestUnsynchronisedRegistry = UnsynchronisedRegistry<
    default::storages::registry_storage::RegistryStorage<ResourceId, TestStoredResource>,
    default::storages::reservation_storage::ReservationStorage<
        ReserverId,
        default::storages::access_storage::AccessStorage<ResourceId, Access>,
    >,
    default::storages::access_storage::AccessStorage<ResourceId, Access>,
    default::storages::credential_storage::CredentialStorage<ReserverId, Password>,
    default::storages::whitelist_storage::WhitelistStorage<ResourceId, Access>,
    default::storages::blacklist_storage::BlacklistStorage<ResourceId, Access, Password>,
    default::storages::control_storage::ControlStorage<ReserverId, ResourceId>,
>;

fn create_registry() -> TestUnsynchronisedRegistry {
    TestUnsynchronisedRegistry::new(
        default::storages::registry_storage::RegistryStorage::new(64),
        Default::default(),
    )
}

#[cfg(feature = "releaser")]
#[test]
fn unsynchronised_releaser_guards_clear_accesses_on_drop() {
    let registry = Arc::new(create_registry());
    let resource_ids = [
        ResourceId::new_label("unsync-release-a"),
        ResourceId::new_label("unsync-release-b"),
    ];
    let owner = ReserverId::new("unsync-release-owner");
    let owner_password = Password::new(73);
    assert!(SynchronisedRegistryRegisterResult::from(registry.register(RegistryRegister {
        id: owner.clone(), password: owner_password.clone(),
    })).ok());
    for resource_id in &resource_ids {
        assert!(SynchronisedRegistryOwnResult::from(registry.own(RegistryOwn {
            id: owner.clone(),
            password: &owner_password,
            resource_id: resource_id.clone(),
        })).ok());
        assert!(SynchronisedRegistryCheckedReplacementResult::from(unsafe {
            registry.checked_replace(RegistryReplacement {
                user_details: Some((&owner, &owner_password)),
                access: &Access::Replace,
                resource_id: resource_id.clone(),
                resource: Some(Resource::new("release-value".to_string())),
                password: None,
            })
        }).ok());
    }

    {
        let _guard = registry.acquire_released_access::<AccessResult<ResourceWrapper<'_, TestStoredResource>>>(RegistryOwnedAcquireAccess {
            user_details: Some((owner.clone(), owner_password.clone())),
            resource_id: resource_ids[0].clone(),
            access: Access::Shared(1),
            password: None,
        }).unwrap_or_else(|_| panic!("unsynchronised released access should succeed"));
    }
    assert_eq!(Some(Access::Shared(0)), registry.get_access(&ReceptionGetAccess { access_id: &resource_ids[0] }));

    {
        let _guard = registry.acquire_released_view::<AccessResult<ResourceView<'_, ResourceId, TestStoredResource>>>(RegistryOwnedAcquireView {
            user_details: Some((owner.clone(), owner_password.clone())),
            resource_details: resource_ids.iter().cloned().map(|id| (id, None)).collect(),
            access: Access::Shared(1),
        }).unwrap_or_else(|_| panic!("unsynchronised released view should succeed"));
    }
    for resource_id in &resource_ids {
        assert_eq!(Some(Access::Shared(0)), registry.get_access(&ReceptionGetAccess { access_id: resource_id }));
    }

    let missing_id = ResourceId::new_label("unsync-release-missing");
    assert!(matches!(registry.acquire_released_access::<AccessResult<ResourceWrapper<'_, TestStoredResource>>>(RegistryOwnedAcquireAccess {
        user_details: None,
        resource_id: missing_id.clone(),
        access: Access::Shared(1),
        password: None,
    }), Err(UnsynchronisedRegistryAcquireAccessError::AutomatedRegistry(_))));
    assert!(matches!(registry.acquire_released_view::<AccessResult<ResourceView<'_, ResourceId, TestStoredResource>>>(RegistryOwnedAcquireView {
        user_details: None,
        resource_details: vec![(missing_id, None)],
        access: Access::Shared(1),
    }), Err(UnsynchronisedRegistryAcquireViewError::AutomatedRegistry(_))));
}

fn contains(registry: &TestUnsynchronisedRegistry, resource_id: &ResourceId) -> bool {
    matches!(
        unsafe { registry.contains_resource(&RegistryContainsResource { resource_id }) },
        UnsynchronisedRegistryContainsResourceResult::AutomatedRegistry(true),
    )
}

fn check_access(
    registry: &TestUnsynchronisedRegistry,
    user_details: Option<(&ReserverId, &Password)>,
    resource_id: &ResourceId,
    access: &Access,
) -> UnsynchronisedRegistryCheckAccessResult {
    unsafe {
        registry.check_access(&RegistryCheckAccess {
            user_details,
            resource_id,
            access,
            password: None,
        })
    }
}

#[test]
fn unsynchronised_registry_runs_its_public_resource_lifecycle() {
    let registry = create_registry();
    let id = ReserverId::new("unsync-owner".to_string());
    let password = Password::new(51);
    let updated_password = Password::new(52);
    let guest = ReserverId::new("unsync-guest".to_string());
    let guest_password = Password::new(53);
    let resource_ids = [
        ResourceId::new_label("unsync-one"),
        ResourceId::new_label("unsync-two"),
    ];

    assert!(SynchronisedRegistryRegisterResult::from(registry.register(RegistryRegister { id: id.clone(), password: password.clone() })).ok());
    assert!(SynchronisedRegistryRegisterResult::from(registry.register(RegistryRegister { id: guest.clone(), password: guest_password.clone() })).ok());
    assert!(registry.registered().contains(&id));
    assert!(SynchronisedRegistryUpdatePasswordResult::from(registry.update_password(RegistryUpdatePassword {
        id: &id,
        old_password: &password,
        new_password: updated_password.clone(),
    })).ok());

    for resource_id in &resource_ids {
        assert!(SynchronisedRegistryOwnResult::from(registry.own(RegistryOwn {
            id: id.clone(),
            password: &updated_password,
            resource_id: resource_id.clone(),
        })).ok());
        assert!(SynchronisedRegistryCheckedReplacementResult::from(unsafe { registry.checked_replace(RegistryReplacement {
            user_details: Some((&id, &updated_password)),
            access: &Access::Replace,
            resource_id: resource_id.clone(),
            resource: Some(Resource::new("value".to_string())),
            password: None,
        }) }).ok());
        assert!(registry.is_owned(&RegistryIsOwned { resource_id }));
        assert!(contains(&registry, resource_id));
    }

    assert_eq!(2, unsafe { registry.len() });
    assert_eq!(2, unsafe { registry.keys() }.len());
    assert!(registry.registered().contains(&id));

    assert!(aion_state::prelude::SynchronisedRegistryWhitelistAllowResult::from(registry.allow_whitelist(RegistryAllow {
        id: &id,
        password: &updated_password,
        resource_id: resource_ids[0].clone(),
        access: Access::Shared(1),
    })).ok());
    assert!(matches!(check_access(&registry, None, &resource_ids[0], &Access::Shared(1)), UnsynchronisedRegistryCheckAccessResult::AutomatedRegistry(true)));
    assert!(matches!(SynchronisedRegistryWhitelistUnallowResult::from(registry.unallow_whitelist(&RegistryUnallow {
        id: &id,
        password: &updated_password,
        resource_id: &resource_ids[0],
        access: &Access::Shared(1),
    })), SynchronisedRegistryWhitelistUnallowResult::Ok));
    assert!(matches!(
        check_access(&registry, None, &resource_ids[0], &Access::Shared(1)),
        UnsynchronisedRegistryCheckAccessResult::Reception(ReceptionCheckAccessResult::Denied(
            OwnerCheckAccessResult::Controller(ControllerCheckAccessResult::AccessControl(_))
        ))
    ));

    assert!(aion_state::prelude::SynchronisedRegistryWhitelistAllowResult::from(registry.allow_whitelist(RegistryAllow {
        id: &id,
        password: &updated_password,
        resource_id: resource_ids[0].clone(),
        access: Access::Shared(1),
    })).ok());
    assert!(matches!(SynchronisedRegistryReservationResult::from(registry.reserve(RegistryReservation {
        id: id.clone(),
        id_password: &updated_password,
        resource_id: resource_ids[0].clone(),
        access: Access::Unique,
        password: None,
    })), SynchronisedRegistryReservationResult::Ok | SynchronisedRegistryReservationResult::OkNew));
    assert!(matches!(
        check_access(&registry, None, &resource_ids[0], &Access::Shared(1)),
        UnsynchronisedRegistryCheckAccessResult::Reception(ReceptionCheckAccessResult::Host(
            HostCheckAccessResult::ReservationConflict
        ))
    ));
    assert!(matches!(SynchronisedRegistryUnreserveResult::from(registry.unreserve(&RegistryUnreserve {
        id: &id,
        password: &updated_password,
        resource_id: &resource_ids[0],
        access: &Access::Unique,
    })), SynchronisedRegistryUnreserveResult::Ok));
    assert!(matches!(SynchronisedRegistryDrainReservationsResult::from(registry.drain_reservations(&RegistryDrainReservations { id: &id, password: &updated_password })),
        SynchronisedRegistryDrainReservationsResult::Drain(reservations)
            if reservations == vec![(resource_ids[0].clone(), Access::Shared(0))]));
    assert!(matches!(check_access(&registry, None, &resource_ids[0], &Access::Shared(1)), UnsynchronisedRegistryCheckAccessResult::AutomatedRegistry(true)));

    assert!(matches!(
        SynchronisedRegistryBlacklistAllowResult::from(registry.allow_blacklist(RegistryAllow {
            id: &id,
            password: &Password::new(0),
            resource_id: resource_ids[1].clone(),
            access: Access::Shared(1),
        })),
        SynchronisedRegistryBlacklistAllowResult::VerificationFailure
    ));
    assert!(matches!(
        SynchronisedRegistryBlacklistAllowResult::from(registry.allow_blacklist(RegistryAllow {
            id: &guest,
            password: &guest_password,
            resource_id: resource_ids[1].clone(),
            access: Access::Shared(1),
        })),
        SynchronisedRegistryBlacklistAllowResult::OwnershipDenied
    ));
    let blacklist_password = match SynchronisedRegistryBlacklistAllowResult::from(registry.allow_blacklist(RegistryAllow {
        id: &id,
        password: &updated_password,
        resource_id: resource_ids[1].clone(),
        access: Access::Shared(1),
    })) {
        SynchronisedRegistryBlacklistAllowResult::Ok(password) => password,
        result => panic!("expected blacklist password, got {result:?}"),
    };
    let check_blacklist = |password| RegistryCheckAccess {
        user_details: Some((&guest, &guest_password)),
        resource_id: &resource_ids[1],
        access: &Access::Shared(1),
        password,
    };
    let wrong_blacklist_password = [0u64, 1].into_iter()
        .map(Password::new)
        .find(|candidate| candidate != &blacklist_password)
        .unwrap();
    for password in [None, Some(&wrong_blacklist_password)] {
        assert!(matches!(
            SynchronisedRegistryCheckAccessResult::from(unsafe {
                registry.check_access(&check_blacklist(password))
            }),
            SynchronisedRegistryCheckAccessResult::ListsDenied
        ));
    }
    assert!(matches!(
        SynchronisedRegistryCheckAccessResult::from(unsafe {
            registry.check_access(&check_blacklist(Some(&blacklist_password)))
        }),
        SynchronisedRegistryCheckAccessResult::ContainsResource
    ));
    assert!(matches!(
        SynchronisedRegistryBlacklistUnallowResult::from(registry.unallow_blacklist(&RegistryUnallow {
            id: &id,
            password: &Password::new(0),
            resource_id: &resource_ids[1],
            access: &Access::Shared(1),
        })),
        SynchronisedRegistryBlacklistUnallowResult::VerificationFailure
    ));
    assert!(matches!(
        SynchronisedRegistryBlacklistUnallowResult::from(registry.unallow_blacklist(&RegistryUnallow {
            id: &guest,
            password: &guest_password,
            resource_id: &resource_ids[1],
            access: &Access::Shared(1),
        })),
        SynchronisedRegistryBlacklistUnallowResult::OwnershipDenied
    ));
    let unallow_blacklist = RegistryUnallow {
        id: &id,
        password: &updated_password,
        resource_id: &resource_ids[1],
        access: &Access::Shared(1),
    };
    assert!(matches!(
        SynchronisedRegistryBlacklistUnallowResult::from(registry.unallow_blacklist(&unallow_blacklist)),
        SynchronisedRegistryBlacklistUnallowResult::Ok
    ));
    assert!(matches!(
        SynchronisedRegistryBlacklistUnallowResult::from(registry.unallow_blacklist(&unallow_blacklist)),
        SynchronisedRegistryBlacklistUnallowResult::Err
    ));

    let access = Access::Shared(1);
    let acquired = unsafe { registry.acquire_access::<AccessResult<
        default::storages::registry_storage::ResourceWrapper<'_, TestStoredResource>,
    >>(RegistryAcquireAccess {
        user_details: Some((&id, &updated_password)),
        resource_id: resource_ids[0].clone(),
        access: access.clone(),
        password: None,
    }) };
    let Ok(acquired) = acquired else { panic!("unsynchronised acquisition should succeed") };
    let AccessResult::Shared(default::storages::registry_storage::ResourceWrapper::Shared(value)) = acquired else {
        panic!("expected a shared resource reference");
    };
    #[cfg(feature = "stable_deref")]
    assert_eq!(**value, Resource::new("value".to_string()));
    #[cfg(not(feature = "stable_deref"))]
    assert_eq!(*value, Resource::new("value".to_string()));
    assert!(matches!(SynchronisedRegistryReleaseAccessResult::from(unsafe { registry.release_access(&RegistryReleaseAccess {
        resource_id: &resource_ids[0],
        access: &access,
    }) }), SynchronisedRegistryReleaseAccessResult::Ok));

    let view = unsafe { registry.acquire_view::<AccessResult<
        default::storages::registry_storage::ResourceView<'_, ResourceId, TestStoredResource>,
    >>(RegistryAcquireView {
        user_details: Some((&id, &updated_password)),
        resource_details: resource_ids.iter().cloned().map(|resource_id| (resource_id, None)).collect(),
        access: access.clone(),
    }) };
    let Ok(view) = view else { panic!("unsynchronised view acquisition should succeed") };
    let AccessResult::Shared(default::storages::registry_storage::ResourceView::Shared(values)) = view else {
        panic!("expected a shared view");
    };
    assert_eq!(values.len(), 2);
    for resource_id in &resource_ids {
        assert!(values.contains_key(resource_id));
    }
    for value in values.values() {
        #[cfg(feature = "stable_deref")]
        assert_eq!(***value, Resource::new("value".to_string()));
        #[cfg(not(feature = "stable_deref"))]
        assert_eq!(**value, Resource::new("value".to_string()));
    }
    drop(values);
    for resource_id in &resource_ids {
        assert!(matches!(SynchronisedRegistryReleaseAccessResult::from(unsafe { registry.release_access(&RegistryReleaseAccess {
            resource_id,
            access: &access,
        }) }), SynchronisedRegistryReleaseAccessResult::Ok));
    }

    assert_eq!(registry.get_access(&ReceptionGetAccess { access_id: &resource_ids[0] }), Some(Access::Shared(0)));
    let missing = ResourceId::new_label("unsync-missing");
    use aion_state::prelude::{ManualRegistryCheckedReplacementResult, ManualRegistryReplacementResult};
    assert!(matches!(unsafe { registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: missing.clone(),
        resource: None,
        password: None,
    }) }, UnsynchronisedRegistryCheckedReplacementResult::AutomatedRegistry(
        ManualRegistryCheckedReplacementResult::ReplacementResult(ManualRegistryReplacementResult::NoOp)
    )));
    assert!(!contains(&registry, &missing));
    let before_denied_insert = unsafe { registry.len() };
    assert!(matches!(unsafe { registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Shared(1),
        resource_id: missing,
        resource: Some(Resource::new("denied".to_string())),
        password: None,
    }) }, UnsynchronisedRegistryCheckedReplacementResult::AutomatedRegistry(
        ManualRegistryCheckedReplacementResult::ReplacementResult(ManualRegistryReplacementResult::DeniedAccess)
    )));
    assert_eq!(before_denied_insert, unsafe { registry.len() });

    assert!(matches!(SynchronisedRegistryReleaseResourceResult::from(registry.release_resource(&RegistryReleaseResource {
        id: &id,
        password: &updated_password,
        resource_id: &resource_ids[0],
    })), SynchronisedRegistryReleaseResourceResult::Ok));
    assert!(!registry.is_owned(&RegistryIsOwned { resource_id: &resource_ids[0] }));
    let release_all = SynchronisedRegistryReleaseResourceAllResult::from(registry.release_resource_all(RegistryReleaseResourceAll {
        id: &id,
        password: &updated_password,
        inputs: vec![&resource_ids[1]],
    }));
    assert!(matches!(release_all, SynchronisedRegistryReleaseResourceAllResult::All(results)
        if results.len() == 1 && matches!(results.first(), Some(SynchronisedRegistryReleaseResourceResult::Ok))));
    assert!(!registry.is_owned(&RegistryIsOwned { resource_id: &resource_ids[1] }));
    assert!(matches!(SynchronisedRegistryUnregisterResult::from(registry.unregister(&RegistryUnregister { id: &id, password: &updated_password })), SynchronisedRegistryUnregisterResult::Lists { .. }));
    assert!(!registry.registered().contains(&id));
    assert_eq!(2, unsafe { registry.len() });
}

#[cfg(feature = "stable_deref")]
#[test]
fn unsynchronised_registry_supports_reallocating_replace_with_stable_storage() {
    use aion_state::synchronised_registry::unsynchronised_registry::unsynchronised_registry_result::UnsynchronisedRegistryReallocatingReplacementResult;

    let registry = create_registry();
    let resource_id = ResourceId::new_label("unsync-stable-value");
    use aion_state::prelude::ManualRegistryReplacementResult;

    let first = unsafe { registry.reallocating_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("first".to_string())),
        password: None,
    }) };
    assert!(matches!(first,
        UnsynchronisedRegistryReallocatingReplacementResult::AutomatedRegistry(
            ManualRegistryReplacementResult::NotFound
        )
    ));

    let replacement = unsafe { registry.reallocating_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: resource_id.clone(),
        resource: Some(Resource::new("second".to_string())),
        password: None,
    }) };
    let UnsynchronisedRegistryReallocatingReplacementResult::AutomatedRegistry(
        ManualRegistryReplacementResult::Found(previous),
    ) = replacement else {
        panic!("expected replacement to return the previous value");
    };
    assert_eq!(previous, Resource::new("first".to_string()));

    let acquired = unsafe { registry.acquire_access::<AccessResult<
        default::storages::registry_storage::ResourceWrapper<'_, TestStoredResource>,
    >>(RegistryAcquireAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: Access::Unique,
        password: None,
    }) };
    let acquired = match acquired {
        Ok(acquired) => acquired,
        Err(_) => panic!("unique access to the replaced resource should succeed"),
    };
    match acquired {
        AccessResult::Unique(default::storages::registry_storage::ResourceWrapper::Unique(stored)) => {
            #[cfg(feature = "stable_deref")]
            assert_eq!(**stored, Resource::new("second".to_string()));
        }
        _ => panic!("expected unique access to the replaced value"),
    }
    assert!(matches!(SynchronisedRegistryReleaseAccessResult::from(unsafe { registry.release_access(&RegistryReleaseAccess {
        resource_id: &resource_id,
        access: &Access::Unique,
    }) }), SynchronisedRegistryReleaseAccessResult::Ok));
    assert_eq!(registry.get_access(&ReceptionGetAccess { access_id: &resource_id }), Some(Access::Shared(0)));
}
