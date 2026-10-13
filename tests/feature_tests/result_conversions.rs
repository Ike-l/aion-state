use std::panic::{AssertUnwindSafe, catch_unwind};

use aion_state::prelude::*;
use crate::default::prelude::{Access, Password, ReserverId, Resource, ResourceId};

fn panics<T>(operation: impl FnOnce() -> T) -> bool {
    catch_unwind(AssertUnwindSafe(operation)).is_err()
}

#[test]
fn check_access_conversion_rejects_unreachable_no_current_access() {
    let result = UnsynchronisedRegistryCheckAccessResult::Reception(ReceptionCheckAccessResult::Host(
        HostCheckAccessResult::Accesses(AccessesCheckAccessResult::NoCurrentAccess),
    ));
    assert!(panics(|| SynchronisedRegistryCheckAccessResult::from(result)));

    for controller_result in [ControllerCheckAccessResult::IsOwner, ControllerCheckAccessResult::NotOwned] {
        let result = UnsynchronisedRegistryCheckAccessResult::Reception(ReceptionCheckAccessResult::Denied(
            OwnerCheckAccessResult::Controller(controller_result),
        ));
        assert!(panics(|| SynchronisedRegistryCheckAccessResult::from(result)));
    }
}

#[test]
fn denied_owner_check_results_are_unreachable_for_reservations_and_acquisitions() {
    for controller_result in [ControllerCheckAccessResult::IsOwner, ControllerCheckAccessResult::NotOwned] {
        assert!(panics(|| SynchronisedRegistryReservationResult::from(
            UnsynchronisedRegistryReservationResult::Reception(ReceptionReservationResult::Denied(
                OwnerCheckAccessResult::Controller(controller_result),
            )),
        )));
    }

    for controller_result in [ControllerCheckAccessResult::IsOwner, ControllerCheckAccessResult::NotOwned] {
        assert!(panics(|| SynchronisedRegistryAcquireAccessError::from(
            UnsynchronisedRegistryAcquireAccessError::Reception(ReceptionCheckAccessResult::Denied(
                OwnerCheckAccessResult::Controller(controller_result),
            )),
        )));
    }
}

#[test]
fn denied_owner_check_results_are_unreachable_for_replacements() {
    for controller_result in [ControllerCheckAccessResult::IsOwner, ControllerCheckAccessResult::NotOwned] {
        let result = UnsynchronisedRegistryReallocatingReplacementResult::<Resource>::Reception(
            ReceptionCheckAccessResult::Denied(OwnerCheckAccessResult::Controller(controller_result)),
        );
        assert!(panics(|| SynchronisedRegistryReallocatingReplacementResult::from(result)));
    }

    for controller_result in [ControllerCheckAccessResult::IsOwner, ControllerCheckAccessResult::NotOwned] {
        let result = UnsynchronisedRegistryCheckedReplacementResult::<Resource>::Reception(
            ReceptionCheckAccessResult::Denied(OwnerCheckAccessResult::Controller(controller_result)),
        );
        assert!(panics(|| SynchronisedRegistryCheckedReplacementResult::from(result)));
    }
}

#[test]
fn release_access_conversion_preserves_the_missing_access_result() {
    let result = UnsynchronisedRegistryReleaseAccessResult::Reception(ReceptionReleaseAccessResult::Host(
        HostReleaseAccessResult::Accesses(AccessesReleaseResult::NoCurrentAccess),
    ));
    assert!(matches!(SynchronisedRegistryReleaseAccessResult::from(result), SynchronisedRegistryReleaseAccessResult::NoCurrentAccess));
}

#[test]
fn result_conversions_preserve_storage_failure_variants() {
    let unregister = UnsynchronisedRegistryUnregisterResult::Reception(ReceptionUnregisterResult::Owner(
        OwnerUnregisterResult::AuthenticatorUnregisterFailure,
    ));
    assert!(matches!(SynchronisedRegistryUnregisterResult::from(unregister), SynchronisedRegistryUnregisterResult::AuthenticatorUnregisterFailure));

    let update_password = UnsynchronisedRegistryUpdatePasswordResult::Reception(ReceptionUpdatePasswordResult::Owner(
        OwnerUpdatePasswordResult::Authenticator(AuthenticateUpdatePasswordResult::Updated(false)),
    ));
    assert!(matches!(SynchronisedRegistryUpdatePasswordResult::from(update_password), SynchronisedRegistryUpdatePasswordResult::Err));

    let own = UnsynchronisedRegistryOwnResult::Reception(ReceptionOwnResult::Owner(OwnerOwnResult::Controller(
        ControllerOwnResult::ResourceControl(ResourceControlOwnResult::Own(false)),
    )));
    assert!(matches!(SynchronisedRegistryOwnResult::from(own), SynchronisedRegistryOwnResult::Err));

    let release = UnsynchronisedRegistryReleaseResourceResult::Reception(ReceptionReleaseResourceResult::Owner(
        OwnerReleaseResourceResult::Controller(ControllerReleaseResourceResult::ResourceControl(
            ResourceControlReleaseResult::Released(false),
        )),
    ));
    assert!(matches!(SynchronisedRegistryReleaseResourceResult::from(release), SynchronisedRegistryReleaseResourceResult::Err));

    let release_all = UnsynchronisedRegistryReleaseResourceAllResult::Reception(ReceptionReleaseResourceAllResult::Owner(
        OwnerReleaseResourceAllResult::Controller(ControllerReleaseResourceAllResult::All(vec![
            ControllerReleaseResourceResult::ResourceControl(ResourceControlReleaseResult::Released(false)),
        ])),
    ));
    assert!(matches!(SynchronisedRegistryReleaseResourceAllResult::from(release_all),
        SynchronisedRegistryReleaseResourceAllResult::All(results)
            if matches!(results.as_slice(), [SynchronisedRegistryReleaseResourceResult::Err])));

    let blacklist_allow = UnsynchronisedRegistryBlacklistAllowResult::<Password>::Reception(ReceptionBlacklistAllowResult::Owner(
        OwnerBlacklistAllowResult::Controller(ControllerBlacklistAllowResult::Blacklist(
            AccessControlBlacklistAllowResult::Blacklist(BlacklistAllowResult::Allow(None)),
        )),
    ));
    assert!(matches!(SynchronisedRegistryBlacklistAllowResult::from(blacklist_allow), SynchronisedRegistryBlacklistAllowResult::Err));

    let whitelist_allow = UnsynchronisedRegistryWhitelistAllowResult::Reception(ReceptionWhitelistAllowResult::Owner(
        OwnerWhitelistAllowResult::Controller(ControllerWhitelistAllowResult::Whitelist(
            AccessControlWhitelistAllowResult::Whitelist(WhitelistAllowResult::Allow(false)),
        )),
    ));
    assert!(matches!(SynchronisedRegistryWhitelistAllowResult::from(whitelist_allow), SynchronisedRegistryWhitelistAllowResult::Err));
}

#[test]
fn replacement_result_ok_distinguishes_mutations_from_refusals() {
    assert!(SynchronisedRegistryReallocatingReplacementResult::<Resource>::Found(Resource::new("old".to_string())).ok());
    assert!(SynchronisedRegistryReallocatingReplacementResult::<Resource>::NotFound.ok());
    assert!(!SynchronisedRegistryReallocatingReplacementResult::<Resource>::NoOp.ok());
    assert!(!SynchronisedRegistryReallocatingReplacementResult::<Resource>::DeniedAccess.ok());

    assert!(SynchronisedRegistryCheckedReplacementResult::<Resource>::Found(Resource::new("old".to_string())).ok());
    assert!(SynchronisedRegistryCheckedReplacementResult::<Resource>::NotFound.ok());
    assert!(!SynchronisedRegistryCheckedReplacementResult::<Resource>::NoOp.ok());
    assert!(!SynchronisedRegistryCheckedReplacementResult::<Resource>::RemovalReallocates.ok());
}

#[derive(Default)]
struct CredentialStoreThatCannotUnregister;

impl CredentialStorage for CredentialStoreThatCannotUnregister {
    type Id = ReserverId;
    type Password = Password;

    fn verify(&self, _id: &Self::Id, _password: &Self::Password) -> bool {
        true
    }

    fn registered(&self) -> impl Iterator<Item = &Self::Id> {
        std::iter::empty()
    }

    fn register(&mut self, _id: Self::Id, _password: Self::Password) -> bool {
        true
    }

    fn update_password(&mut self, _id: &Self::Id, _new_password: Self::Password) -> bool {
        true
    }

    fn unregister(&mut self, _id: &Self::Id) -> bool {
        false
    }
}

#[test]
fn owner_reports_credential_store_unregister_failure() {
    type TestOwner = Owner<
        CredentialStoreThatCannotUnregister,
        crate::default::storages::whitelist_storage::WhitelistStorage<ResourceId, Access>,
        crate::default::storages::blacklist_storage::BlacklistStorage<ResourceId, Access, Password>,
        crate::default::storages::control_storage::ControlStorage<ReserverId, ResourceId>,
    >;

    let mut owner = TestOwner::new(Default::default());
    let id = ReserverId::new("unregister-failure");
    let password = Password::new(1);
    let result = owner.unregister(&OwnerUnregister { id: &id, password: &password });
    assert!(matches!(result, OwnerUnregisterResult::AuthenticatorUnregisterFailure));
}

#[test]
fn reception_record_access_reports_a_denied_identity() {
    type TestReception = Reception<
        crate::default::storages::reservation_storage::ReservationStorage<
            ReserverId,
            crate::default::storages::access_storage::AccessStorage<ResourceId, Access>,
        >,
        crate::default::storages::access_storage::AccessStorage<ResourceId, Access>,
        crate::default::storages::credential_storage::CredentialStorage<ReserverId, Password>,
        crate::default::storages::whitelist_storage::WhitelistStorage<ResourceId, Access>,
        crate::default::storages::blacklist_storage::BlacklistStorage<ResourceId, Access, Password>,
        crate::default::storages::control_storage::ControlStorage<ReserverId, ResourceId>,
    >;

    let mut reception = TestReception::new(Default::default());
    let resource_id = ResourceId::new_label("reception-record-access");
    let access = Access::Shared(1);
    assert!(reception.record_access(ReceptionRecordAccess {
        user_details: None,
        resource_id: resource_id.clone(),
        access: access.clone(),
        password: None,
    }).ok());

    let unregistered_id = ReserverId::new("reception-unregistered");
    let unregistered_password = Password::new(2);
    let denied = reception.record_access(ReceptionRecordAccess {
        user_details: Some((&unregistered_id, &unregistered_password)),
        resource_id: resource_id.clone(),
        access: access.clone(),
        password: None,
    });
    assert!(matches!(denied, ReceptionRecordAccessResult::Denied(OwnerCheckAccessResult::Denied)));
    assert!(!denied.ok());
    assert!(matches!(reception.release_access(&ReceptionReleaseAccess {
        resource_id: &resource_id, access: &access,
    }), ReceptionReleaseAccessResult::Host(HostReleaseAccessResult::Accesses(AccessesReleaseResult::Split))));
}
