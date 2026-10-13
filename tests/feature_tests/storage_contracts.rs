use aion_state::prelude::{
    Accesses, AccessesCheckAccess,
    AccessesRecordAccess, AccessesRelease, AccessesReleaseResult, ControlStorage as ControlStorageTrait,
    CredentialStorage as CredentialStorageTrait, GetAccess, RegistryStorage as RegistryStorageTrait,
    RegistryReplacement, ReservationStorage as ReservationStorageTrait,
    WhitelistStorage as WhitelistStorageTrait,
    BlacklistStorage as BlacklistStorageTrait,
    AccessControl, AccessControlAllow, AccessControlBlacklistAllowResult,
    AccessControlCheckAccess, AccessControlRelease,
    AccessControlReleaseResult, BlacklistReleaseResult,
    AccessControlWhitelistAllowResult,
    Reservations, ReservationsReservation, ReservationsReserveResult, ReservationsUnreserve,
    ReservationsUnreserveResult, ResourceControl, ResourceControlCheckOwner,
    ResourceControlOwn, ResourceControlRelease, ResourceControlReleaseResult,
    SynchronisedRegistryCheckedReplacementResult,
};

use crate::default::prelude::*;

#[test]
fn checked_replacement_reports_insert_and_remove_reallocation_refusals() {
    let insert_registry = crate::create_registry(Some(0));
    let insert_id = ResourceId::new_label("checked-insert-reallocation");
    assert!(matches!(insert_registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: insert_id,
        resource: Some(Resource::new("value".to_string())),
        password: None,
    }), SynchronisedRegistryCheckedReplacementResult::InsertingReallocates));

    let storage = RegistryStorage::<ResourceId, StoredResource>::new_with_removal_reallocation(4, true);
    let remove_registry = crate::TestRegistry::new(storage, Default::default());
    let remove_id = ResourceId::new_label("checked-remove-reallocation");
    assert!(remove_registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: remove_id.clone(),
        resource: Some(Resource::new("value".to_string())),
        password: None,
    }).ok());
    assert!(matches!(remove_registry.checked_replace(RegistryReplacement {
        user_details: None,
        access: &Access::Replace,
        resource_id: remove_id,
        resource: None,
        password: None,
    }), SynchronisedRegistryCheckedReplacementResult::RemovalReallocates));
}

#[test]
fn resource_storage_tracks_insert_replace_view_and_remove() {
    let mut storage = RegistryStorage::<ResourceId, StoredResource>::new(1);
    let id = ResourceId::new_label("storage-resource");

    assert!(!unsafe { storage.next_insert_may_reallocates() });
    assert!(storage.insert(id.clone(), Resource::new("first".to_string())).is_none());
    assert!(storage.contains_key(&id));
    assert_eq!(1, storage.len());
    assert!(unsafe { storage.next_insert_may_reallocates() });

    let replaced = storage.insert(id.clone(), Resource::new("second".to_string()));
    assert_eq!(Some(Resource::new("first".to_string())), replaced);
    let view = storage.get_view(&[&id]).unwrap();
    assert_eq!(view.get(&id), Some(&Resource::new("second".to_string())));
    drop(view);

    let missing = ResourceId::new_label("missing-view-resource");
    assert!(storage.get_view(&[&id, &missing]).is_none());

    assert_eq!(Some(Resource::new("second".to_string())), storage.remove(&id));
    assert!(!storage.contains_key(&id));
    assert_eq!(0, storage.len());
    assert!(storage.get_view(&[&id]).is_none());
    assert!(!unsafe { storage.next_removal_may_reallocates() });
}

#[test]
fn access_storage_and_accesses_cover_missing_record_merge_release_and_drain() {
    let id = ResourceId::new_label("access-storage");
    let mut accesses = Accesses::<AccessStorage<ResourceId, Access>>::default();

    assert!(matches!(accesses.check_access(&AccessesCheckAccess { access_id: &id, access: &Access::Unique }), aion_state::prelude::AccessesCheckAccessResult::NoCurrentAccess));
    assert!(matches!(accesses.record_access(AccessesRecordAccess { access_id: id.clone(), access: Access::Shared(1) }), aion_state::prelude::AccessesRecordAccessResult::Inserted));
    assert!(matches!(accesses.check_access(&AccessesCheckAccess { access_id: &id, access: &Access::Shared(2) }), aion_state::prelude::AccessesCheckAccessResult::Ok(true)));
    assert!(matches!(accesses.record_access(AccessesRecordAccess { access_id: id.clone(), access: Access::Shared(2) }), aion_state::prelude::AccessesRecordAccessResult::Merged));
    assert_eq!(Some(&Access::Shared(3)), accesses.get_access(&GetAccess { access_id: &id }));
    let released = accesses.release(&AccessesRelease { access_id: &id, access: &Access::Shared(1) });
    assert!(matches!(released, AccessesReleaseResult::Split));
    assert!(released.ok());
    assert_eq!(Some(&Access::Shared(2)), accesses.get_access(&GetAccess { access_id: &id }));

    let aion_state::prelude::AccessesDrainResult::Drain(drained) = accesses.drain();
    assert_eq!(vec![(id.clone(), Access::Shared(2))], drained);
    let missing_release = accesses.release(&AccessesRelease { access_id: &id, access: &Access::Shared(1) });
    assert!(matches!(missing_release, AccessesReleaseResult::NoCurrentAccess));
    assert!(!missing_release.ok());
}

#[test]
fn reservation_storage_inserts_replaces_reads_and_iterates() {
    let reserver = ReserverId::new("reservation-storage".to_string());
    let mut storage = ReservationStorage::<ReserverId, AccessStorage<ResourceId, Access>>::default();
    assert!(storage.get_mut(&reserver).is_none());

    let previous = storage.insert(reserver.clone(), Accesses::default());
    assert!(previous.is_none());
    assert!(storage.get_mut(&reserver).is_some());
    assert_eq!(1, storage.iter().count());

    let replaced = storage.insert(reserver.clone(), Accesses::default());
    assert!(replaced.is_some());
    assert_eq!(1, storage.iter().count());
}

#[test]
fn reservations_unreserve_ok_reflects_the_access_release_result() {
    type TestReservations = Reservations<ReservationStorage<
        ReserverId,
        AccessStorage<ResourceId, Access>,
    >>;

    let mut reservations = TestReservations::default();
    let reserver_id = ReserverId::new("reservations-result".to_string());
    let resource_id = ResourceId::new_label("reservations-result-resource");
    assert!(matches!(reservations.reserve(ReservationsReservation {
        reserver_id: reserver_id.clone(),
        access_id: resource_id.clone(),
        access: Access::Shared(1),
    }), ReservationsReserveResult::FirstReservation));

    let released = reservations.unreserve(&ReservationsUnreserve {
        reserver_id: &reserver_id,
        access_id: &resource_id,
        access: &Access::Shared(1),
    });
    assert!(matches!(released, ReservationsUnreserveResult::Accesses(AccessesReleaseResult::Split)));
    assert!(released.ok());

    let missing = reservations.unreserve(&ReservationsUnreserve {
        reserver_id: &reserver_id,
        access_id: &ResourceId::new_label("not-reserved"),
        access: &Access::Shared(1),
    });
    assert!(matches!(missing, ReservationsUnreserveResult::Accesses(AccessesReleaseResult::NoCurrentAccess)));
    assert!(!missing.ok());
}

#[test]
fn resource_control_checks_owner_sets_and_release_results() {
    let owner = ReserverId::new("resource-control-owner".to_string());
    let other = ReserverId::new("resource-control-other".to_string());
    let resource_id = ResourceId::new_label("resource-control-owned");
    let mut control = ResourceControl::<ControlStorage<ReserverId, ResourceId>>::default();

    assert!(matches!(control.own(ResourceControlOwn {
        id: owner.clone(),
        resource_id: resource_id.clone(),
    }), aion_state::prelude::ResourceControlOwnResult::Own(true)));
    assert!(control.check_owners(std::iter::empty::<ResourceControlCheckOwner<'_, ReserverId, ResourceId>>()).ok());
    assert!(control.check_owners([ResourceControlCheckOwner { id: &owner, resource_id: &resource_id }].into_iter()).ok());
    let denied = control.check_owners([ResourceControlCheckOwner { id: &other, resource_id: &resource_id }].into_iter());
    assert!(matches!(denied, aion_state::prelude::ResourceControlCheckOwnersResult::Invalid(Some(_))));
    assert!(!denied.ok());

    let released = control.release(&ResourceControlRelease { resource_id: &resource_id });
    assert!(matches!(released, ResourceControlReleaseResult::Released(true)));
    assert!(released.ok());
    let missing = control.release(&ResourceControlRelease { resource_id: &resource_id });
    assert!(matches!(missing, ResourceControlReleaseResult::Released(false)));
    assert!(!missing.ok());
}

#[test]
fn access_control_release_checks_both_lists_and_reports_missing_entries() {
    type TestAccessControl = AccessControl<
        crate::default::storages::whitelist_storage::WhitelistStorage<ResourceId, Access>,
        crate::default::storages::blacklist_storage::BlacklistStorage<ResourceId, Access, Password>,
    >;

    let mut access_control = TestAccessControl::new(Default::default());
    let resource_id = ResourceId::new_label("access-control-release");
    let access = Access::Shared(1);
    assert!(matches!(access_control.allow_whitelist(AccessControlAllow {
        id: resource_id.clone(), access: access.clone(),
    }), AccessControlWhitelistAllowResult::Whitelist(_)));
    let blacklist_password = match access_control.allow_blacklist(AccessControlAllow {
        id: resource_id.clone(), access: access.clone(),
    }) {
        AccessControlBlacklistAllowResult::Blacklist(aion_state::prelude::BlacklistAllowResult::Allow(Some(password))) => password,
        _ => panic!("blacklist storage should generate a password"),
    };

    let released = access_control.release(&AccessControlRelease { id: &resource_id });
    assert!(released.ok());
    assert!(!access_control.check_access(&AccessControlCheckAccess {
        id: &resource_id, access: &access, password: None,
    }).ok());
    assert!(!access_control.check_access(&AccessControlCheckAccess {
        id: &resource_id, access: &access, password: Some(&blacklist_password),
    }).ok());

    let missing = access_control.release(&AccessControlRelease {
        id: &ResourceId::new_label("access-control-missing"),
    });
    assert!(!missing.ok());
    assert!(matches!(missing, AccessControlReleaseResult::Lists((
        aion_state::prelude::WhitelistReleaseResult::Release(false),
        BlacklistReleaseResult::Release(false),
    ))));
}

#[test]
fn credentials_and_control_storages_handle_duplicate_missing_and_release_cases() {
    let id = ReserverId::new("credential-storage".to_string());
    let password = Password::new(71);
    let updated = Password::new(72);
    let resource_id = ResourceId::new_label("control-storage");

    let mut credentials = CredentialStorage::<ReserverId, Password>::default();
    assert!(!credentials.verify(&id, &password));
    assert!(credentials.register(id.clone(), password.clone()));
    assert!(!credentials.register(id.clone(), password.clone()));
    assert!(credentials.verify(&id, &password));
    assert!(!credentials.verify(&id, &updated));
    assert!(!credentials.update_password(&ReserverId::new("missing".to_string()), updated.clone()));
    assert!(credentials.update_password(&id, updated.clone()));
    assert!(!credentials.verify(&id, &password));
    assert!(credentials.verify(&id, &updated));
    assert_eq!(vec![id.clone()], credentials.registered().cloned().collect::<Vec<_>>());
    assert!(credentials.unregister(&id));
    assert!(!credentials.unregister(&id));

    let mut controls = ControlStorage::<ReserverId, ResourceId>::default();
    assert!(!controls.is_owned(&resource_id));
    assert!(controls.own(id.clone(), resource_id.clone()));
    assert!(controls.is_owned(&resource_id));
    assert!(controls.check_owner(&id, &resource_id));
    assert!(!controls.check_owner(&ReserverId::new("other".to_string()), &resource_id));
    assert_eq!(vec![resource_id.clone()], controls.release_id(&id).collect::<Vec<_>>());
    assert!(!controls.is_owned(&resource_id));
    assert!(!controls.release(&resource_id));
}

#[test]
fn whitelist_and_blacklist_release_all_visit_every_id() {
    let missing = ResourceId::new_label("missing-list");
    let present = ResourceId::new_label("present-list");
    let access = Access::Shared(1);

    let mut whitelist = WhitelistStorage::<ResourceId, Access>::default();
    assert!(whitelist.allow(present.clone(), access.clone()));
    assert!(!whitelist.release_all([&missing, &present].into_iter()));
    assert!(!whitelist.check_access(&present, &access));

    let mut blacklist = BlacklistStorage::<ResourceId, Access, Password>::default();
    let password = blacklist.allow(present.clone(), access.clone()).unwrap();
    assert!(blacklist.check_access(&present, &access, &password));
    let wrong_password = [0u64, 1].into_iter().map(Password::new).find(|candidate| candidate != &password).unwrap();
    assert!(!blacklist.check_access(&present, &access, &wrong_password));
    assert!(!blacklist.release_all([&missing, &present].into_iter()));
    assert!(!blacklist.check_access(&present, &access, &password));
}
