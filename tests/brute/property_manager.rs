use std::{collections::{HashMap, HashSet}, mem::discriminant, sync::Arc};
#[cfg(feature = "notifier")]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(any(feature = "async", feature = "notifier"))]
use std::{future::Future, task::{Context, Poll, Wake, Waker}};

use aion_state::prelude::{Accessor, ReceptionGetAccess, RegistryAcquireAccess, RegistryAcquireView, RegistryAllow, RegistryCheckAccess, RegistryCheckOwner, RegistryCheckedReplacementResult, RegistryContainsResource, RegistryDrainReservations, RegistryIsOwned, RegistryOwn, RegistryRegister, RegistryReleaseAccess, RegistryReleaseResource, RegistryReleaseResourceAll, RegistryReplacement, RegistryReservation, RegistryUnallow, RegistryUnregister, RegistryUnreserve, RegistryUpdatePassword, SynchronisedRegistryAcquireAccessError, SynchronisedRegistryCheckAccessResult, SynchronisedRegistryReleaseResourceAllResult};
#[cfg(any(feature = "releaser", feature = "notifier"))]
use aion_state::prelude::RegistryOwnedAcquireAccess;
#[cfg(feature = "releaser")]
use aion_state::prelude::{RegistryOwnedAcquireView, Releaser, ViewReleaser};
#[cfg(all(feature = "async", feature = "releaser"))]
use aion_state::prelude::{AsyncReleaser, AsyncViewReleaser};
#[cfg(feature = "notifier")]
use aion_state::prelude::{FutureAcquireAccess, PermissiveFilter};
use tracing::{Level, event};

use crate::{BruteRegistry as TestRegistry, BruteStoredResource, brute::command::Command, default::{prelude::{Access, AccessResult, Password, ReserverId, Resource, ResourceId}, storages::registry_storage::{ResourceView, ResourceWrapper}}};
#[cfg(feature = "async")]
use crate::block_on;

#[cfg(any(feature = "releaser", feature = "notifier", feature = "async"))]
type BruteReferencedValue<'a> = ResourceWrapper<'a, BruteStoredResource>;
#[cfg(any(feature = "releaser", feature = "notifier", feature = "async"))]
type BruteAccessResult<'a> = AccessResult<BruteReferencedValue<'a>>;
#[cfg(any(feature = "releaser", feature = "async"))]
type BruteView<'a> = ResourceView<'a, ResourceId, BruteStoredResource>;
#[cfg(any(feature = "releaser", feature = "async"))]
type BruteViewResult<'a> = AccessResult<BruteView<'a>>;

#[cfg(feature = "notifier")]
struct BruteCountingWake(AtomicUsize);

#[cfg(feature = "notifier")]
impl Wake for BruteCountingWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(feature = "notifier")]
fn brute_counting_waker() -> (Waker, Arc<BruteCountingWake>) {
    let counter = Arc::new(BruteCountingWake(AtomicUsize::new(0)));
    (Waker::from(counter.clone()), counter)
}

#[cfg(feature = "notifier")]
type BruteNotifierFuture<'a> = FutureAcquireAccess<
    'a,
    BruteReferencedValue<'a>,
    TestRegistry,
    PermissiveFilter<SynchronisedRegistryAcquireAccessError>,
    ReserverId,
    Password,
    ResourceId,
    Access,
    Password,
    BruteAccessResult<'a>,
>;

pub struct PropertyManager {
    registered: HashSet<ReserverId>,
    owned_resources: HashMap<ReserverId, HashSet<ResourceId>>,
    known_owned_resource_ids: HashSet<ResourceId>,
    resources: HashSet<ResourceId>,
    resource_values: HashMap<ResourceId, Resource>,
    known_resource_ids: HashSet<ResourceId>,
    whitelist: HashMap<ResourceId, Vec<Access>>,
    blacklist: HashMap<ResourceId, Vec<(Access, Password)>>,
    reservations: HashMap<ReserverId, HashMap<ResourceId, Access>>,
    active_accesses: HashMap<ResourceId, Access>,
    command_counts: [usize; Command::LEN],
    release_access_count: usize,
}

impl PropertyManager {
    pub fn new() -> Self {
        Self {
            registered: HashSet::new(),
            owned_resources: HashMap::new(),
            known_owned_resource_ids: HashSet::new(),
            resources: HashSet::new(),
            resource_values: HashMap::new(),
            known_resource_ids: HashSet::new(),
            whitelist: HashMap::new(),
            blacklist: HashMap::new(),
            reservations: HashMap::new(),
            active_accesses: HashMap::new(),
            command_counts: [0; Command::LEN],
            release_access_count: 0,
        }
    }

    pub fn owned_resource_ids(&self, id: &ReserverId) -> Vec<ResourceId> {
        self.owned_resources.get(id).map(|resources| resources.iter().cloned().collect()).unwrap_or_default()
    }

    pub fn test(&mut self, registry: &Arc<TestRegistry>, command: Command) {
        event!(Level::INFO, "Executing Command: {command:?}");
        self.command_counts[command.index()] += 1;
        match command {
            Command::CheckedReplacement{ user_details, credentials_match, access, resource_id, resource, password } => self.test_checked_replacement(registry, user_details, credentials_match, access, resource_id, resource, password),
            Command::Register { id, password } => self.test_register(registry, id, password),
            Command::Unregister { id, password, credentials_match } => self.test_unregister(registry, id, password, credentials_match),
            Command::Own { id, password, credentials_match, resource_id } => self.test_own(registry, id, password, credentials_match, resource_id),
            Command::UpdatePassword { id, old_password, credentials_match, new_password } => self.test_update_password(registry, id, old_password, credentials_match, new_password),
            Command::ReleaseResource { id, password, credentials_match, resource_id } => self.test_release_resource(registry, id, password, credentials_match, resource_id),
            Command::ReleaseResourceAll { id, password, credentials_match, resource_ids } => self.test_release_resource_all(registry, id, password, credentials_match, resource_ids),
            Command::AllowBlacklist { id, password, credentials_match, resource_id, access } => self.test_allow_blacklist(registry, id, password, credentials_match, resource_id, access),
            Command::AllowWhitelist { id, password, credentials_match, resource_id, access } => self.test_allow_whitelist(registry, id, password, credentials_match, resource_id, access),
            Command::UnallowBlacklist { id, password, credentials_match, resource_id, access } => self.test_unallow_blacklist(registry, id, password, credentials_match, resource_id, access),
            Command::UnallowWhitelist { id, password, credentials_match, resource_id, access } => self.test_unallow_whitelist(registry, id, password, credentials_match, resource_id, access),
            Command::CheckAccess { user_details, credentials_match, resource_id, access, password } => self.test_check_access(registry, user_details, credentials_match, resource_id, access, password),
            Command::Reserve { id, password, credentials_match, resource_id } => self.test_reserve(registry, id, password, credentials_match, resource_id),
            Command::Unreserve { id, password, credentials_match, resource_id } => self.test_unreserve(registry, id, password, credentials_match, resource_id),
            Command::DrainReservations { id, password, credentials_match } => self.test_drain_reservations(registry, id, password, credentials_match),
            Command::AcquireAccess { user_details, credentials_match, resource_id, access, password } => self.test_acquire_access(registry, user_details, credentials_match, resource_id, access, password),
            Command::AcquireView { user_details, credentials_match, resource_access, access } => self.test_acquire_view(registry, user_details, credentials_match, resource_access, access),
            Command::ReallocatingReplacement { user_details, credentials_match, resource_id, resource, password } => self.test_reallocating_replacement(registry, user_details, credentials_match, resource_id, resource, password),
            Command::ContainsResource { resource_id } => self.test_contains_resource(registry, resource_id),
            Command::IsOwned { resource_id } => self.test_is_owned(registry, resource_id),
            Command::CheckOwner { id, resource_id } => self.test_check_owner(registry, id, resource_id),
            Command::Len => assert_eq!(registry.len(), self.resources.len()),
            Command::GetAccess { resource_id } => self.test_get_access(registry, resource_id),
            Command::Keys => self.test_keys(registry),
            Command::Registered => self.test_registered(registry),
            #[cfg(feature = "releaser")]
            Command::Releaser => self.test_releaser(registry),
            #[cfg(feature = "notifier")]
            Command::Notifier => self.test_notifier(registry),
            #[cfg(feature = "async")]
            Command::Async => self.test_async_api(registry),
        }
    }

    pub fn assert_consistent(&self, registry: &TestRegistry) {
        assert!(self.command_counts.iter().all(|count| *count > 0), "At least one brute command was not exercised: {:?}", self.command_counts);
        assert!(self.release_access_count > 0);
        assert_eq!(self.registered, registry.registered().into_iter().collect());
        assert_eq!(self.resources, registry.keys().into_iter().collect());
        assert_eq!(self.resources.len(), registry.len());
        assert_eq!(self.resources, self.resource_values.keys().cloned().collect());

        for (id, resource_ids) in &self.owned_resources {
            assert!(self.registered.contains(id));
            for resource_id in resource_ids {
                assert!(registry.check_owner(&RegistryCheckOwner { id, resource_id }).ok());
                assert!(registry.is_owned(&RegistryIsOwned { resource_id }));
            }
        }

        for resource_id in &self.known_owned_resource_ids {
            let owner = self.owned_resources
                .iter()
                .find_map(|(id, resource_ids)| resource_ids.contains(resource_id).then_some(id));
            assert_eq!(owner.is_some(), registry.is_owned(&RegistryIsOwned { resource_id }));
            if let Some(id) = owner {
                assert!(registry.check_owner(&RegistryCheckOwner { id, resource_id }).ok());
            }
        }

        for resource_id in &self.known_resource_ids {
            assert_eq!(self.resources.contains(resource_id), registry.contains_resource(&RegistryContainsResource { resource_id }).ok());
            assert_eq!(self.active_accesses.get(resource_id).cloned(), registry.get_access(&ReceptionGetAccess { access_id: resource_id }));
        }
    }

    fn test_checked_replacement(
        &mut self,
        registry: &Arc<TestRegistry>,
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        access: Access,
        resource_id: ResourceId,
        resource: Option<Resource>,
        password: Option<Password>
    ) {
        self.known_resource_ids.insert(resource_id.clone());
        let registry_contains_resource = RegistryContainsResource { resource_id: &resource_id };
        let contains = registry.contains_resource(&registry_contains_resource).ok();
        assert_eq!(self.resources.contains(&resource_id), contains);

        let resource_is_some = resource.is_some();
        let expected_resource = resource.clone();
        let expected_old_resource = self.resource_values.get(&resource_id).cloned();

        let resources_before = registry.len();
        let access_check = self.expected_check_access(
            user_details.as_ref().map(|(id, password)| (id, password)),
            credentials_match,
            &resource_id,
            &access,
            password.as_ref(),
        );
        let authorized = matches!(access, Access::Replace)
            && matches!(access_check, SynchronisedRegistryCheckAccessResult::ContainsResource | SynchronisedRegistryCheckAccessResult::MissingResource);

        let input = RegistryReplacement {
            user_details: user_details.as_ref().map(|(id, password)| (id, password)),
            access: &access,
            resource_id: resource_id.clone(),
            resource,
            password: password.as_ref(),
        };

        let result = registry.checked_replace(input);

        if !authorized {
            match access_check {
                SynchronisedRegistryCheckAccessResult::Err => assert!(matches!(result, RegistryCheckedReplacementResult::AccessConflict)),
                SynchronisedRegistryCheckAccessResult::ReservationConflict => assert!(matches!(result, RegistryCheckedReplacementResult::ReservationConflict)),
                SynchronisedRegistryCheckAccessResult::VerificationFailure => assert!(matches!(result, RegistryCheckedReplacementResult::VerificationFailure)),
                SynchronisedRegistryCheckAccessResult::ListsDenied => assert!(matches!(result, RegistryCheckedReplacementResult::ListsDenied)),
                SynchronisedRegistryCheckAccessResult::ContainsResource | SynchronisedRegistryCheckAccessResult::MissingResource => {
                    if !resource_is_some && !contains {
                        assert!(matches!(result, RegistryCheckedReplacementResult::NoOp),
                            "expected NoOp for absent removal; got {result:?} (access={access:?}, check={access_check:?}, id={resource_id:?})");
                    } else {
                        assert!(matches!(result, RegistryCheckedReplacementResult::DeniedAccess),
                            "expected DeniedAccess; got {result:?} (resource_is_some={resource_is_some}, contains={contains}, access={access:?}, check={access_check:?}, id={resource_id:?})");
                    }
                }
            }
            self.assert_resource_unchanged(registry, resources_before, contains, &registry_contains_resource);
            return
        }

        let mutation_succeeded = result.ok();
        self.assert_checked_replacement_mutation(
            registry, 
            result,
            contains,
            resource_is_some,
            resources_before,
            expected_old_resource,
            &registry_contains_resource
        );

        if mutation_succeeded {
            if let Some(expected_resource) = expected_resource.as_ref() {
                let probe = registry.checked_replace(RegistryReplacement {
                    user_details: user_details.as_ref().map(|(id, password)| (id, password)),
                    access: &access,
                    resource_id: resource_id.clone(),
                    resource: Some(expected_resource.clone()),
                    password: password.as_ref(),
                });
                let RegistryCheckedReplacementResult::Found(previous) = probe else {
                    panic!("replacing the stored resource should return its previous value");
                };
                assert_eq!(previous, *expected_resource);
            }
            if let Some(expected_resource) = expected_resource {
                self.resources.insert(resource_id.clone());
                self.resource_values.insert(resource_id, expected_resource);
            } else {
                self.resources.remove(&resource_id);
                self.resource_values.remove(&resource_id);
            }
        }
    }

    fn assert_resource_unchanged(
        &self,
        registry: &Arc<TestRegistry>,
        resources_before: usize,
        contained_before: bool,
        registry_contains_resource: &RegistryContainsResource<'_, ResourceId>,
    ) {
        assert_eq!(resources_before, registry.len());
        assert_eq!(contained_before, registry.contains_resource(registry_contains_resource).ok());
    }

    fn assert_checked_replacement_mutation(
        &self,
        registry: &Arc<TestRegistry>,
        result: RegistryCheckedReplacementResult<Resource>,
        contains: bool,
        resource_is_some: bool,
        resources_before: usize,
        expected_old_resource: Option<Resource>,
        registry_contains_resource: &RegistryContainsResource<'_, ResourceId> 
    ) {
        if !resource_is_some {
            if contains {
                let RegistryCheckedReplacementResult::Found(previous) = result else {
                    panic!("removing a stored resource should return its previous value");
                };
                assert_eq!(Some(previous), expected_old_resource);
                assert_eq!(resources_before - 1, registry.len());
                assert!(!registry.contains_resource(registry_contains_resource).ok());
            } else {
                assert!(matches!(result, RegistryCheckedReplacementResult::NoOp));
                self.assert_resource_unchanged(registry, resources_before, contains, registry_contains_resource);
            }

            return
        }

        if contains {
            let RegistryCheckedReplacementResult::Found(previous) = &result else {
                panic!("replacing a stored resource should return its previous value");
            };
            assert_eq!(Some(previous.clone()), expected_old_resource);
            assert_eq!(resources_before, registry.len());
        } else if result.ok() {
            assert!(matches!(&result, RegistryCheckedReplacementResult::NotFound));
            assert_eq!(resources_before + 1, registry.len());
        } else {
            assert!(matches!(result, RegistryCheckedReplacementResult::InsertingReallocates));
            assert_eq!(resources_before, registry.len());
        }

        assert_eq!(
            contains || (resource_is_some && result.ok()),
            registry.contains_resource(registry_contains_resource).ok()
        );
    }

    fn test_register(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password
    ) {
        let already_registered = self.registered.contains(&id);
        
        let result = registry.register(RegistryRegister {
            id: id.clone(),
            password,
        });

        assert!(registry.registered().contains(&id));
        assert_eq!(!already_registered, result.ok());
        self.registered.insert(id);
    }

    fn test_unregister(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
    ) {
        let input = RegistryUnregister {
            id: &id,
            password: &password,
        };

        let registered = self.registered.contains(&id);
        assert_eq!(registered, registry.registered().contains(&id));

        let result = registry.unregister(&input);

        let should_unregister = registered && credentials_match;
        assert_eq!(should_unregister, result.ok());
        assert_eq!(registered && !should_unregister, registry.registered().contains(&id));

        if should_unregister {
            self.registered.remove(&id);
            if let Some(resources) = self.owned_resources.remove(&id) {
                for resource_id in resources {
                    assert!(!registry.check_owner(&RegistryCheckOwner { id: &id, resource_id: &resource_id }).ok());
                    assert!(!registry.is_owned(&RegistryIsOwned { resource_id: &resource_id }));
                    self.whitelist.remove(&resource_id);
                    self.blacklist.remove(&resource_id);
                }
            }
        } else if let Some(resources) = self.owned_resources.get(&id) {
            for resource_id in resources {
                assert!(registry.check_owner(&RegistryCheckOwner { id: &id, resource_id }).ok());
                assert!(registry.is_owned(&RegistryIsOwned { resource_id }));
            }
        }
    }

    fn test_own(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId
    ) {
        self.known_owned_resource_ids.insert(resource_id.clone());
        self.known_resource_ids.insert(resource_id.clone());

        let registered = self.registered.contains(&id);
        assert_eq!(registered, registry.registered().contains(&id));

        let input = RegistryIsOwned {
            resource_id: &resource_id
        };

        let already_owned = registry.is_owned(&input);
        let owned_by_user = registry.check_owner(&RegistryCheckOwner { id: &id, resource_id: &resource_id }).ok();

        let result = registry.own(RegistryOwn {
            id: id.clone(),
            password: &password,
            resource_id: resource_id.clone(),
        });

        let should_own = registered && credentials_match && !already_owned;
        assert_eq!(should_own, result.ok());
        assert_eq!(already_owned || should_own, registry.is_owned(&input));
        assert_eq!(owned_by_user || should_own, registry.check_owner(&RegistryCheckOwner { id: &id, resource_id: &resource_id }).ok());

        if should_own {
            self.owned_resources.entry(id).or_default().insert(resource_id);
        }
    }

    fn owns(&self, id: &ReserverId, resource_id: &ResourceId) -> bool {
        self.owned_resources.get(id).is_some_and(|resources| resources.contains(resource_id))
    }

    fn owns_any(&self, resource_id: &ResourceId) -> bool {
        self.owned_resources.values().any(|resources| resources.contains(resource_id))
    }

    fn release_control(&mut self, id: &ReserverId, resource_id: &ResourceId) {
        if let Some(resources) = self.owned_resources.get_mut(id) {
            resources.remove(resource_id);
            if resources.is_empty() {
                self.owned_resources.remove(id);
            }
        }
        self.whitelist.remove(resource_id);
        self.blacklist.remove(resource_id);
    }

    fn test_update_password(
        &self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        old_password: Password,
        credentials_match: bool,
        new_password: Password,
    ) {
        let should_update = self.registered.contains(&id) && credentials_match;
        let result = registry.update_password(RegistryUpdatePassword {
            id: &id,
            old_password: &old_password,
            new_password,
        });
        assert_eq!(should_update, result.ok());
    }

    fn test_release_resource(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
    ) {
        self.known_owned_resource_ids.insert(resource_id.clone());
        self.known_resource_ids.insert(resource_id.clone());
        let should_release = self.registered.contains(&id) && credentials_match && self.owns(&id, &resource_id);
        let result = registry.release_resource(&RegistryReleaseResource {
            id: &id,
            password: &password,
            resource_id: &resource_id,
        });
        assert_eq!(should_release, result.ok());
        assert_eq!(!should_release && self.owns(&id, &resource_id), registry.check_owner(&RegistryCheckOwner { id: &id, resource_id: &resource_id }).ok());
        if should_release {
            self.release_control(&id, &resource_id);
        }
        assert_eq!(self.owns_any(&resource_id), registry.is_owned(&RegistryIsOwned { resource_id: &resource_id }));
    }

    fn test_release_resource_all(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_ids: Vec<ResourceId>,
    ) {
        for resource_id in &resource_ids {
            self.known_owned_resource_ids.insert(resource_id.clone());
            self.known_resource_ids.insert(resource_id.clone());
        }
        let can_release = self.registered.contains(&id) && credentials_match;
        let result = registry.release_resource_all(RegistryReleaseResourceAll {
            id: &id,
            password: &password,
            inputs: resource_ids.iter().collect(),
        });
        match (can_release, result) {
            (true, SynchronisedRegistryReleaseResourceAllResult::All(results)) => {
                assert_eq!(resource_ids.len(), results.len());
                for (resource_id, result) in resource_ids.iter().zip(results) {
                    let should_release = self.owns(&id, resource_id);
                    assert_eq!(should_release, result.ok());
                    if should_release {
                        self.release_control(&id, resource_id);
                    }
                }
            }
            (false, SynchronisedRegistryReleaseResourceAllResult::VerificationFailure) => {}
            _ => panic!("Unexpected release-resource-all result"),
        }
    }

    fn test_allow_blacklist(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
    ) {
        self.known_owned_resource_ids.insert(resource_id.clone());
        self.known_resource_ids.insert(resource_id.clone());
        let expected = self.registered.contains(&id) && credentials_match && self.owns(&id, &resource_id);
        let result = registry.allow_blacklist(RegistryAllow {
            id: &id,
            password: &password,
            resource_id: resource_id.clone(),
            access: access.clone(),
        });
        assert_eq!(expected, result.ok());
        if let aion_state::prelude::SynchronisedRegistryBlacklistAllowResult::Ok(blacklist_password) = result {
            self.blacklist.entry(resource_id.clone()).or_default().push((access.clone(), blacklist_password.clone()));
            let check = self.expected_check_access(None, true, &resource_id, &access, Some(&blacklist_password));
            let actual = registry.check_access(&RegistryCheckAccess {
                user_details: None,
                resource_id: &resource_id,
                access: &access,
                password: Some(&blacklist_password),
            });
            assert_eq!(discriminant(&check), discriminant(&actual));
        }
    }

    fn test_allow_whitelist(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
    ) {
        self.known_owned_resource_ids.insert(resource_id.clone());
        self.known_resource_ids.insert(resource_id.clone());
        let expected = self.registered.contains(&id) && credentials_match && self.owns(&id, &resource_id);
        let result = registry.allow_whitelist(RegistryAllow {
            id: &id,
            password: &password,
            resource_id: resource_id.clone(),
            access: access.clone(),
        });
        assert_eq!(expected, result.ok());
        if expected {
            self.whitelist.entry(resource_id).or_default().push(access);
        }
    }

    fn test_unallow_blacklist(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
    ) {
        let list_has_access = self.blacklist.get(&resource_id).is_some_and(|entries| entries.iter().any(|(entry_access, _)| entry_access == &access));
        let may_unallow = self.registered.contains(&id) && credentials_match && self.owns(&id, &resource_id);
        let result = registry.unallow_blacklist(&RegistryUnallow { id: &id, password: &password, resource_id: &resource_id, access: &access });
        let unallowed = matches!(result, aion_state::prelude::SynchronisedRegistryBlacklistUnallowResult::Ok);
        assert_eq!(may_unallow && list_has_access, unallowed, "blacklist unallow mismatch: id={id:?}, resource={resource_id:?}, access={access:?}, credential_match={credentials_match}, list_has_access={list_has_access}, expected_owner={:?}, entries={:?}", self.owner_of(&resource_id), self.blacklist.get(&resource_id));
        if unallowed {
            let entries = self.blacklist.get_mut(&resource_id).unwrap();
            let index = entries.iter().position(|(entry_access, _)| entry_access == &access).unwrap();
            entries.remove(index);
        }
    }

    fn test_unallow_whitelist(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
    ) {
        let list_has_access = self.whitelist.get(&resource_id).is_some_and(|entries| entries.contains(&access));
        let may_unallow = self.registered.contains(&id) && credentials_match && self.owns(&id, &resource_id);
        let result = registry.unallow_whitelist(&RegistryUnallow { id: &id, password: &password, resource_id: &resource_id, access: &access });
        let unallowed = matches!(result, aion_state::prelude::SynchronisedRegistryWhitelistUnallowResult::Ok);
        assert_eq!(may_unallow && list_has_access, unallowed, "whitelist unallow mismatch: id={id:?}, resource={resource_id:?}, access={access:?}, credential_match={credentials_match}, list_has_access={list_has_access}, expected_owner={:?}, entries={:?}", self.owner_of(&resource_id), self.whitelist.get(&resource_id));
        if unallowed {
            let entries = self.whitelist.get_mut(&resource_id).unwrap();
            let index = entries.iter().position(|entry_access| entry_access == &access).unwrap();
            entries.remove(index);
        }
    }

    fn expected_check_access(
        &self,
        user_details: Option<(&ReserverId, &Password)>,
        credentials_match: bool,
        resource_id: &ResourceId,
        access: &Access,
        password: Option<&Password>,
    ) -> SynchronisedRegistryCheckAccessResult {
        if user_details.is_some_and(|(id, _)| !credentials_match || !self.registered.contains(id)) {
            return SynchronisedRegistryCheckAccessResult::VerificationFailure;
        }

        let owner = self.owner_of(resource_id);
        let is_owner = user_details.is_some_and(|(id, _)| owner == Some(id));
        if owner.is_some() && !is_owner {
            let whitelisted = self.whitelist.get(resource_id).is_some_and(|entries| entries.contains(access));
            let blacklisted = password.is_some_and(|password| {
                self.blacklist.get(resource_id).is_some_and(|entries| {
                    entries.iter().any(|(entry_access, entry_password)| entry_access == access && entry_password == password)
                })
            });
            if !whitelisted && !blacklisted {
                return SynchronisedRegistryCheckAccessResult::ListsDenied;
            }
        }

        let reserver_id = user_details.map(|(id, _)| id);
        let reservation_conflict = self.reservations.iter().any(|(id, accesses)| {
            if Some(id) == reserver_id {
                return false;
            }
            accesses.get(resource_id).is_some_and(|current| !current.accepts_incoming(access))
        });
        if reservation_conflict {
            return SynchronisedRegistryCheckAccessResult::ReservationConflict;
        }

        if self.active_accesses.get(resource_id).is_some_and(|current| !current.accepts_incoming(access)) {
            return SynchronisedRegistryCheckAccessResult::Err;
        }

        if self.resources.contains(resource_id) {
            SynchronisedRegistryCheckAccessResult::ContainsResource
        } else {
            SynchronisedRegistryCheckAccessResult::MissingResource
        }
    }

    fn test_check_access(
        &mut self,
        registry: &Arc<TestRegistry>,
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
        password: Option<Password>,
    ) {
        self.known_resource_ids.insert(resource_id.clone());
        let expected = self.expected_check_access(
            user_details.as_ref().map(|(id, password)| (id, password)),
            credentials_match,
            &resource_id,
            &access,
            password.as_ref(),
        );
        let actual = registry.check_access(&RegistryCheckAccess {
            user_details: user_details.as_ref().map(|(id, password)| (id, password)),
            resource_id: &resource_id,
            access: &access,
            password: password.as_ref(),
        });
        assert_eq!(discriminant(&expected), discriminant(&actual), "check-access mismatch for {resource_id:?} {access:?}: expected {expected:?}, got {actual:?}");
    }

    fn owner_of(&self, resource_id: &ResourceId) -> Option<&ReserverId> {
        self.owned_resources.iter().find_map(|(id, resources)| resources.contains(resource_id).then_some(id))
    }

    #[cfg(any(feature = "releaser", feature = "notifier", feature = "async"))]
    fn unowned_feature_resource(&mut self, registry: &Arc<TestRegistry>, excluded: &[ResourceId]) -> Option<ResourceId> {
        if let Some(resource_id) = self.resources.iter().find(|resource_id| {
            !excluded.contains(resource_id) && !self.owns_any(resource_id)
        }).cloned() {
            return Some(resource_id)
        }

        let mut index = 0usize;
        loop {
            let resource_id = ResourceId::new_label(format!("brute-feature-resource-{index}"));
            if !self.known_resource_ids.contains(&resource_id) && !excluded.contains(&resource_id) {
                let result = registry.checked_replace(RegistryReplacement {
                    user_details: None,
                    access: &Access::Replace,
                    resource_id: resource_id.clone(),
                    resource: Some(Resource::new("feature-resource".to_string())),
                    password: None,
                });
                if result.ok() {
                    self.resources.insert(resource_id.clone());
                    self.resource_values.insert(resource_id.clone(), Resource::new("feature-resource".to_string()));
                    self.known_resource_ids.insert(resource_id.clone());
                    return Some(resource_id)
                }
                return self.resources.iter().find(|resource_id| {
                    !excluded.contains(resource_id) && !self.owns_any(resource_id)
                }).cloned()
            }
            index += 1;
        }
    }

    #[cfg(feature = "async")]
    fn fresh_feature_resource(&mut self, registry: &Arc<TestRegistry>, excluded: &[ResourceId]) -> Option<ResourceId> {
        let mut index = self.known_resource_ids.len();
        loop {
            let resource_id = ResourceId::new_label(format!("brute-async-resource-{index}"));
            if !self.known_resource_ids.contains(&resource_id) && !excluded.contains(&resource_id) {
                let result = registry.checked_replace(RegistryReplacement {
                    user_details: None,
                    access: &Access::Replace,
                    resource_id: resource_id.clone(),
                    resource: Some(Resource::new("async-resource".to_string())),
                    password: None,
                });
                if result.ok() {
                    self.resources.insert(resource_id.clone());
                    self.resource_values.insert(resource_id.clone(), Resource::new("async-resource".to_string()));
                    self.known_resource_ids.insert(resource_id.clone());
                    return Some(resource_id)
                }
                return self.unowned_feature_resource(registry, excluded)
            }
            index += 1;
        }
    }

    fn test_reserve(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
    ) {
        self.known_resource_ids.insert(resource_id.clone());
        let check = self.expected_check_access(Some((&id, &password)), credentials_match, &resource_id, &Access::Shared(1), None);
        let result = registry.reserve(RegistryReservation {
            id: id.clone(),
            id_password: &password,
            resource_id: resource_id.clone(),
            access: Access::Shared(1),
            password: None,
        });
        let can_reserve = matches!(check, SynchronisedRegistryCheckAccessResult::ContainsResource | SynchronisedRegistryCheckAccessResult::MissingResource);
        assert_eq!(can_reserve, matches!(result, aion_state::prelude::SynchronisedRegistryReservationResult::Ok | aion_state::prelude::SynchronisedRegistryReservationResult::OkNew));

        if can_reserve {
            let reservations = self.reservations.entry(id.clone()).or_default();
            match reservations.get_mut(&resource_id) {
                Some(current) => current.merge(Access::Shared(1)),
                None => {
                    reservations.insert(resource_id.clone(), Access::Shared(1));
                }
            }

            let probe = self.expected_check_access(None, true, &resource_id, &Access::Unique, None);
            let actual_probe = registry.check_access(&RegistryCheckAccess {
                user_details: None,
                resource_id: &resource_id,
                access: &Access::Unique,
                password: None,
            });
            assert_eq!(discriminant(&probe), discriminant(&actual_probe));

            let unreserve = registry.unreserve(&RegistryUnreserve {
                id: &id,
                password: &password,
                resource_id: &resource_id,
                access: &Access::Shared(1),
            });
            assert!(matches!(unreserve, aion_state::prelude::SynchronisedRegistryUnreserveResult::Ok));
            self.reservations.get_mut(&id).unwrap().get_mut(&resource_id).unwrap().release(&Access::Shared(1));

            self.drain_and_check(registry, &id, &password, credentials_match);
        }
    }

    fn drain_and_check(&mut self, registry: &Arc<TestRegistry>, id: &ReserverId, password: &Password, credentials_match: bool) {
        let expected_reservations = self.reservations.get(id).cloned();
        let result = registry.drain_reservations(&RegistryDrainReservations { id, password });
        if self.registered.contains(id) && credentials_match {
            match (expected_reservations, result) {
                (Some(expected), aion_state::prelude::SynchronisedRegistryDrainReservationsResult::Drain(actual)) => {
                    let actual: HashMap<_, _> = actual.into_iter().collect();
                    assert_eq!(expected, actual);
                }
                (None, aion_state::prelude::SynchronisedRegistryDrainReservationsResult::NoReserver) => {}
                _ => panic!("Unexpected drain-reservations result"),
            }
            if let Some(reservations) = self.reservations.get_mut(id) {
                reservations.clear();
            }
        } else {
            assert!(matches!(result, aion_state::prelude::SynchronisedRegistryDrainReservationsResult::VerificationFailure));
        }
    }

    fn test_unreserve(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
    ) {
        self.known_resource_ids.insert(resource_id.clone());
        let is_registered = self.registered.contains(&id);
        let expected_access = self.reservations.get(&id).and_then(|entries| entries.get(&resource_id)).cloned();
        let result = registry.unreserve(&RegistryUnreserve { id: &id, password: &password, resource_id: &resource_id, access: &Access::Shared(1) });
        match (is_registered && credentials_match, expected_access, result) {
            (false, _, aion_state::prelude::SynchronisedRegistryUnreserveResult::VerificationFailure) => {}
            (true, Some(Access::Shared(count)), aion_state::prelude::SynchronisedRegistryUnreserveResult::Ok) if count > 0 => {
                self.reservations.get_mut(&id).unwrap().get_mut(&resource_id).unwrap().release(&Access::Shared(1));
            }
            (true, Some(Access::Shared(0)), aion_state::prelude::SynchronisedRegistryUnreserveResult::NoReservation) => {}
            (true, None, aion_state::prelude::SynchronisedRegistryUnreserveResult::NoReservation) if self.reservations.contains_key(&id) => {}
            (true, None, aion_state::prelude::SynchronisedRegistryUnreserveResult::NoReservationsMadeByReserver) if !self.reservations.contains_key(&id) => {}
            _ => panic!("Unexpected unreserve result"),
        }
    }

    fn test_drain_reservations(
        &mut self,
        registry: &Arc<TestRegistry>,
        id: ReserverId,
        password: Password,
        credentials_match: bool,
    ) {
        self.drain_and_check(registry, &id, &password, credentials_match);
    }

    fn test_contains_resource(&mut self, registry: &Arc<TestRegistry>, resource_id: ResourceId) {
        self.known_resource_ids.insert(resource_id.clone());
        assert_eq!(self.resources.contains(&resource_id), registry.contains_resource(&RegistryContainsResource { resource_id: &resource_id }).ok());
    }

    fn test_is_owned(&mut self, registry: &Arc<TestRegistry>, resource_id: ResourceId) {
        self.known_owned_resource_ids.insert(resource_id.clone());
        assert_eq!(self.owns_any(&resource_id), registry.is_owned(&RegistryIsOwned { resource_id: &resource_id }));
    }

    fn test_check_owner(&mut self, registry: &Arc<TestRegistry>, id: ReserverId, resource_id: ResourceId) {
        self.known_owned_resource_ids.insert(resource_id.clone());
        let expected = self.owns(&id, &resource_id);
        assert_eq!(expected, registry.check_owner(&RegistryCheckOwner { id: &id, resource_id: &resource_id }).ok());
    }

    fn test_get_access(&mut self, registry: &Arc<TestRegistry>, resource_id: ResourceId) {
        self.known_resource_ids.insert(resource_id.clone());
        let expected = self.active_accesses.get(&resource_id).cloned();
        assert_eq!(expected, registry.get_access(&ReceptionGetAccess { access_id: &resource_id }));
    }

    fn test_keys(&self, registry: &Arc<TestRegistry>) {
        assert_eq!(self.resources, registry.keys().into_iter().collect());
    }

    fn test_registered(&self, registry: &Arc<TestRegistry>) {
        assert_eq!(self.registered, registry.registered().into_iter().collect());
    }

    #[cfg(feature = "releaser")]
    fn test_releaser(&mut self, registry: &Arc<TestRegistry>) {
        let Some(resource_id) = self.unowned_feature_resource(registry, &[]) else { return };
        let access = Access::Shared(1);
        let guard = registry.acquire_released_access::<BruteAccessResult<'_>>(RegistryOwnedAcquireAccess {
            user_details: None,
            resource_id: resource_id.clone(),
            access: access.clone(),
            password: None,
        }).expect("releaser access should succeed for an unowned resource");
        assert!(guard.as_ref().is_some());
        self.record_active_access(registry, &resource_id, &access);
        drop(guard);
        self.release_access_after_guard_drop(registry, &resource_id, &access);

        let Some(second_resource_id) = self.unowned_feature_resource(registry, &[resource_id.clone()]) else { return };
        let resource_ids = vec![resource_id, second_resource_id];
        let guard = registry.acquire_released_view::<BruteViewResult<'_>>(RegistryOwnedAcquireView {
            user_details: None,
            resource_access: resource_ids.iter().cloned().map(|resource_id| (resource_id, None)).collect(),
            access: access.clone(),
        }).expect("releaser view should succeed for stored unowned resources");
        assert!(guard.as_ref().is_some());
        for resource_id in &resource_ids {
            self.record_active_access(registry, resource_id, &access);
        }
        drop(guard);
        for resource_id in &resource_ids {
            self.release_access_after_guard_drop(registry, resource_id, &access);
        }
    }

    #[cfg(feature = "notifier")]
    fn test_notifier(&mut self, registry: &Arc<TestRegistry>) {
        let Some(resource_id) = self.unowned_feature_resource(registry, &[]) else { return };
        let access = Access::Shared(1);
        let held = registry.acquire_access::<BruteAccessResult<'_>>(RegistryAcquireAccess {
            user_details: None,
            resource_id: resource_id.clone(),
            access: access.clone(),
            password: None,
        }).expect("initial shared access should succeed");
        self.record_active_access(registry, &resource_id, &access);

        let input = RegistryOwnedAcquireAccess {
            user_details: None,
            resource_id: resource_id.clone(),
            access: access.clone(),
            password: None,
        };
        let mut future = Box::pin(BruteNotifierFuture::new(
            registry.as_ref(),
            input,
            PermissiveFilter::default(),
        ));
        let (waker, wake_count) = brute_counting_waker();
        let mut context = Context::from_waker(&waker);
        assert!(matches!(future.as_mut().poll(&mut context), Poll::Pending));

        drop(held);
        self.release_access_from_registry(registry, &resource_id, &access);
        assert!(wake_count.0.load(Ordering::Relaxed) > 0, "releasing access should wake the notifier waiter");

        let acquired = match future.as_mut().poll(&mut context) {
            Poll::Ready(Ok(acquired)) => acquired,
            Poll::Ready(Err(error)) => panic!("notifier acquisition failed after release: {error:?}"),
            Poll::Pending => panic!("notifier did not wake the waiting future"),
        };
        assert!(matches!(acquired, AccessResult::Shared(ResourceWrapper::Shared(_))));
        drop(acquired);
        self.record_active_access(registry, &resource_id, &access);
        drop(future);
        self.release_access_from_registry(registry, &resource_id, &access);
    }

    #[cfg(feature = "async")]
    fn test_async_api(&mut self, registry: &Arc<TestRegistry>) {
        let Some(resource_id1) = self.fresh_feature_resource(registry, &[]) else { return };
        let Some(resource_id2) = self.fresh_feature_resource(registry, &[resource_id1.clone()]) else { return };

        let mut suffix = self.registered.len();
        let id = loop {
            let id = ReserverId::new(format!("brute-async-{suffix}"));
            if !self.registered.contains(&id) {
                break id
            }
            suffix += 1;
        };
        let password = Password::new(suffix as u64);
        let updated_password = Password::new((suffix + 1) as u64);

        assert!(block_on(registry.register_async(RegistryRegister { id: id.clone(), password: password.clone() })).ok());
        self.registered.insert(id.clone());
        assert!(block_on(registry.update_password_async(RegistryUpdatePassword {
            id: &id,
            old_password: &password,
            new_password: updated_password.clone(),
        })).ok());

        for resource_id in [&resource_id1, &resource_id2] {
            assert!(block_on(registry.own_async(RegistryOwn {
                id: id.clone(),
                password: &updated_password,
                resource_id: resource_id.clone(),
            })).ok());
            self.known_owned_resource_ids.insert(resource_id.clone());
            self.known_resource_ids.insert(resource_id.clone());
            self.owned_resources.entry(id.clone()).or_default().insert(resource_id.clone());

            let expected_old = self.resource_values.get(resource_id).cloned();
            let result = block_on(registry.checked_replace_async(RegistryReplacement {
                user_details: Some((&id, &updated_password)),
                access: &Access::Replace,
                resource_id: resource_id.clone(),
                resource: Some(Resource::new("async-value".to_string())),
                password: None,
            }));
            let aion_state::prelude::SynchronisedRegistryCheckedReplacementResult::Found(previous) = result else {
                panic!("async replacement should return the existing resource value: {result:?}");
            };
            assert_eq!(Some(previous), expected_old);
            self.resource_values.insert(resource_id.clone(), Resource::new("async-value".to_string()));
            self.resources.insert(resource_id.clone());
        }

        #[cfg(feature = "stable_deref")]
        {
            let expected_old = self.resource_values.get(&resource_id1).cloned();
            let result = block_on(registry.reallocating_replace_async(RegistryReplacement {
            user_details: Some((&id, &updated_password)),
            access: &Access::Replace,
            resource_id: resource_id1.clone(),
            resource: Some(Resource::new("async-reallocated".to_string())),
            password: None,
            }));
            let aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::Found(previous) = result else {
                panic!("async stable replacement should return the previous value");
            };
            assert_eq!(Some(previous), expected_old);
            self.resource_values.insert(resource_id1.clone(), Resource::new("async-reallocated".to_string()));
        }

        #[cfg(not(feature = "stable_deref"))]
        {
            let expected_old = self.resource_values.get(&resource_id1).cloned();
            let result = block_on(registry.checked_replace_async(RegistryReplacement {
            user_details: Some((&id, &updated_password)),
            access: &Access::Replace,
            resource_id: resource_id1.clone(),
            resource: Some(Resource::new("async-replaced".to_string())),
            password: None,
            }));
            let aion_state::prelude::SynchronisedRegistryCheckedReplacementResult::Found(previous) = result else {
                panic!("async replacement should return the previous value");
            };
            assert_eq!(Some(previous), expected_old);
            self.resource_values.insert(resource_id1.clone(), Resource::new("async-replaced".to_string()));
        }

        assert!(block_on(registry.allow_whitelist_async(RegistryAllow {
            id: &id,
            password: &updated_password,
            resource_id: resource_id1.clone(),
            access: Access::Shared(1),
        })).ok());
        self.whitelist.entry(resource_id1.clone()).or_default().push(Access::Shared(1));
        let blacklist_password = match block_on(registry.allow_blacklist_async(RegistryAllow {
            id: &id,
            password: &updated_password,
            resource_id: resource_id1.clone(),
            access: Access::Unique,
        })) {
            aion_state::prelude::SynchronisedRegistryBlacklistAllowResult::Ok(password) => password,
            result => panic!("Unexpected async blacklist allow result: {result:?}"),
        };
        self.blacklist.entry(resource_id1.clone()).or_default().push((Access::Unique, blacklist_password.clone()));

        let expected_access = self.expected_check_access(None, true, &resource_id1, &Access::Shared(1), None);
        let actual_access = block_on(registry.check_access_async(&RegistryCheckAccess {
            user_details: None,
            resource_id: &resource_id1,
            access: &Access::Shared(1),
            password: None,
        }));
        assert_eq!(discriminant(&expected_access), discriminant(&actual_access));

        assert!(matches!(block_on(registry.unallow_whitelist_async(&RegistryUnallow {
            id: &id,
            password: &updated_password,
            resource_id: &resource_id1,
            access: &Access::Shared(1),
        })), aion_state::prelude::SynchronisedRegistryWhitelistUnallowResult::Ok));
        self.whitelist.get_mut(&resource_id1).unwrap().remove(0);
        assert!(matches!(block_on(registry.unallow_blacklist_async(&RegistryUnallow {
            id: &id,
            password: &updated_password,
            resource_id: &resource_id1,
            access: &Access::Unique,
        })), aion_state::prelude::SynchronisedRegistryBlacklistUnallowResult::Ok));
        self.blacklist.get_mut(&resource_id1).unwrap().clear();

        assert!(matches!(block_on(registry.reserve_async(RegistryReservation {
            id: id.clone(),
            id_password: &updated_password,
            resource_id: resource_id1.clone(),
            access: Access::Shared(1),
            password: None,
        })), aion_state::prelude::SynchronisedRegistryReservationResult::Ok | aion_state::prelude::SynchronisedRegistryReservationResult::OkNew));
        let reservations = self.reservations.entry(id.clone()).or_default();
        reservations.insert(resource_id1.clone(), Access::Shared(1));
        assert!(matches!(block_on(registry.unreserve_async(&RegistryUnreserve {
            id: &id,
            password: &updated_password,
            resource_id: &resource_id1,
            access: &Access::Shared(1),
        })), aion_state::prelude::SynchronisedRegistryUnreserveResult::Ok));
        self.reservations.get_mut(&id).unwrap().get_mut(&resource_id1).unwrap().release(&Access::Shared(1));
        let drained = block_on(registry.drain_reservations_async(&RegistryDrainReservations { id: &id, password: &updated_password }));
        let aion_state::prelude::SynchronisedRegistryDrainReservationsResult::Drain(drained) = drained else { panic!("Expected async reservation drain") };
        assert_eq!(self.reservations.get(&id).unwrap().clone(), drained.into_iter().collect());
        self.reservations.get_mut(&id).unwrap().clear();

        for resource_id in [&resource_id1, &resource_id2] {
            assert!(block_on(registry.contains_resource_async(&RegistryContainsResource { resource_id })).ok());
            assert!(block_on(registry.is_owned_async(&RegistryIsOwned { resource_id })));
            assert!(block_on(registry.check_owner_async(&RegistryCheckOwner { id: &id, resource_id })).ok());
        }
        assert_eq!(self.resources.len(), block_on(registry.len_async()));
        assert_eq!(self.resources, block_on(registry.keys_async()).into_iter().collect());
        assert_eq!(self.registered, block_on(registry.registered_async()).into_iter().collect());

        let access = Access::Shared(1);
        let acquired = block_on(registry.acquire_access_async::<BruteAccessResult<'_>>(RegistryAcquireAccess {
            user_details: Some((&id, &updated_password)),
            resource_id: resource_id1.clone(),
            access: access.clone(),
            password: None,
        })).expect("async acquisition should succeed");
        drop(acquired);
        self.record_active_access(registry, &resource_id1, &access);
        self.release_access_async(registry, &resource_id1, &access);

        let acquired_view = block_on(registry.acquire_view_async::<BruteViewResult<'_>>(RegistryAcquireView {
            user_details: Some((&id, &updated_password)),
            resource_access: vec![(resource_id1.clone(), None), (resource_id2.clone(), None)],
            access: access.clone(),
        })).expect("async view acquisition should succeed");
        assert!(matches!(&acquired_view, AccessResult::Shared(ResourceView::Shared(_))));
        drop(acquired_view);
        for resource_id in [&resource_id1, &resource_id2] {
            self.record_active_access(registry, resource_id, &access);
            self.release_access_async(registry, resource_id, &access);
            assert_eq!(Some(Access::Shared(0)), block_on(registry.get_access_async(&ReceptionGetAccess { access_id: resource_id })));
        }

        #[cfg(feature = "releaser")]
        {
            let guard = block_on(registry.async_acquire_released_access::<BruteAccessResult<'_>>(RegistryOwnedAcquireAccess {
                user_details: Some((id.clone(), updated_password.clone())),
                resource_id: resource_id1.clone(),
                access: access.clone(),
                password: None,
            })).expect("async releaser access should succeed");
            self.record_active_access(registry, &resource_id1, &access);
            drop(guard);
            self.release_access_after_guard_drop(registry, &resource_id1, &access);

            let guard = block_on(registry.async_acquire_released_view::<BruteViewResult<'_>>(RegistryOwnedAcquireView {
                user_details: Some((id.clone(), updated_password.clone())),
                resource_access: vec![(resource_id1.clone(), None), (resource_id2.clone(), None)],
                access: access.clone(),
            })).expect("async releaser view should succeed");
            for resource_id in [&resource_id1, &resource_id2] {
                self.record_active_access(registry, resource_id, &access);
            }
            drop(guard);
            for resource_id in [&resource_id1, &resource_id2] {
                self.release_access_after_guard_drop(registry, resource_id, &access);
            }
        }

        let release_result = block_on(registry.release_resource_async(&RegistryReleaseResource {
            id: &id,
            password: &updated_password,
            resource_id: &resource_id2,
        }));
        assert!(release_result.ok());
        self.release_control(&id, &resource_id2);

        let release_all = block_on(registry.release_resource_all_async(RegistryReleaseResourceAll {
            id: &id,
            password: &updated_password,
            inputs: vec![&resource_id1],
        }));
        let aion_state::prelude::SynchronisedRegistryReleaseResourceAllResult::All(results) = release_all else { panic!("Expected async release-all results") };
        assert_eq!(results.len(), 1);
        assert!(results[0].ok());
        self.release_control(&id, &resource_id1);

        assert!(block_on(registry.unregister_async(&RegistryUnregister { id: &id, password: &updated_password })).ok());
        self.registered.remove(&id);
    }

    fn test_acquire_access(
        &mut self,
        registry: &Arc<TestRegistry>,
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
        password: Option<Password>,
    ) {
        self.known_resource_ids.insert(resource_id.clone());
        let expected = self.expected_check_access(
            user_details.as_ref().map(|(id, password)| (id, password)),
            credentials_match,
            &resource_id,
            &access,
            password.as_ref(),
        );
        let expected_error = match expected {
            SynchronisedRegistryCheckAccessResult::Err => Some(SynchronisedRegistryAcquireAccessError::AccessConflict),
            SynchronisedRegistryCheckAccessResult::ReservationConflict => Some(SynchronisedRegistryAcquireAccessError::ReservationConflict),
            SynchronisedRegistryCheckAccessResult::VerificationFailure => Some(SynchronisedRegistryAcquireAccessError::VerificationFailure),
            SynchronisedRegistryCheckAccessResult::ListsDenied => Some(SynchronisedRegistryAcquireAccessError::ListsDenied),
            SynchronisedRegistryCheckAccessResult::MissingResource => Some(SynchronisedRegistryAcquireAccessError::NotFound),
            SynchronisedRegistryCheckAccessResult::ContainsResource if matches!(access, Access::Shared(0) | Access::Replace) => Some(SynchronisedRegistryAcquireAccessError::TriedAcquiring),
            SynchronisedRegistryCheckAccessResult::ContainsResource => None,
        };
        let result = registry.acquire_access::<AccessResult<ResourceWrapper<'_, BruteStoredResource>>>(RegistryAcquireAccess {
            user_details: user_details.as_ref().map(|(id, password)| (id, password)),
            resource_id: resource_id.clone(),
            access: access.clone(),
            password: password.as_ref(),
        });

        match (expected_error, result) {
            (None, Ok(acquired)) => {
                match (&access, &acquired) {
                    (Access::Shared(_), AccessResult::Shared(ResourceWrapper::Shared(_))) => {}
                    (Access::Unique, AccessResult::Unique(ResourceWrapper::Unique(_))) => {}
                    _ => panic!("acquisition result did not match requested access {access:?}"),
                }
                drop(acquired);
                self.record_and_release_access(registry, &resource_id, access);
            }
            (Some(expected), Err(actual)) => assert_eq!(expected, actual),
            (expected_error, actual) => panic!("Acquire outcome mismatch: expected {expected_error:?}, got {actual:?}"),
        }
    }

    fn record_active_access(&mut self, registry: &Arc<TestRegistry>, resource_id: &ResourceId, access: &Access) {
        self.active_accesses.entry(resource_id.clone())
            .and_modify(|current| current.merge(access.clone()))
            .or_insert(access.clone());
        assert_eq!(self.active_accesses.get(resource_id).cloned(), registry.get_access(&ReceptionGetAccess { access_id: resource_id }));
    }

    fn release_access_from_registry(&mut self, registry: &Arc<TestRegistry>, resource_id: &ResourceId, access: &Access) {
        let result = unsafe { registry.release_access(&RegistryReleaseAccess { resource_id, access }) };
        assert!(matches!(result, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
        self.release_access_after_guard_drop(registry, resource_id, access);
    }

    fn release_access_after_guard_drop(&mut self, registry: &Arc<TestRegistry>, resource_id: &ResourceId, access: &Access) {
        self.release_access_count += 1;
        self.active_accesses.get_mut(resource_id).unwrap().release(access);
        assert_eq!(self.active_accesses.get(resource_id).cloned(), registry.get_access(&ReceptionGetAccess { access_id: resource_id }));
    }

    #[cfg(feature = "async")]
    fn release_access_async(&mut self, registry: &Arc<TestRegistry>, resource_id: &ResourceId, access: &Access) {
        let result = unsafe {
            block_on(registry.release_access_async(&RegistryReleaseAccess { resource_id, access }))
        };
        assert!(matches!(result, aion_state::prelude::SynchronisedRegistryReleaseAccessResult::Ok));
        self.release_access_after_guard_drop(registry, resource_id, access);
    }

    fn record_and_release_access(&mut self, registry: &Arc<TestRegistry>, resource_id: &ResourceId, access: Access) {
        self.record_active_access(registry, resource_id, &access);
        self.release_access_from_registry(registry, resource_id, &access);
    }

    fn test_acquire_view(
        &mut self,
        registry: &Arc<TestRegistry>,
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        resource_access: Vec<(ResourceId, Option<Password>)>,
        access: Access,
    ) {
        for (resource_id, _) in &resource_access {
            self.known_resource_ids.insert(resource_id.clone());
        }
        let checks: Vec<_> = resource_access.iter().map(|(resource_id, password)| {
            self.expected_check_access(
                user_details.as_ref().map(|(id, password)| (id, password)),
                credentials_match,
                resource_id,
                &access,
                password.as_ref(),
            )
        }).collect();
        let duplicate_ids = resource_access.iter().enumerate().filter_map(|(index, (resource_id, _))| {
            resource_access[..index].iter().any(|(prior_id, _)| prior_id == resource_id).then_some(index)
        }).collect::<Vec<_>>();
        let denied = checks.iter().enumerate().filter_map(|(index, check)| {
            let error = match check {
                SynchronisedRegistryCheckAccessResult::Err => Some(SynchronisedRegistryAcquireAccessError::AccessConflict),
                SynchronisedRegistryCheckAccessResult::ReservationConflict => Some(SynchronisedRegistryAcquireAccessError::ReservationConflict),
                SynchronisedRegistryCheckAccessResult::VerificationFailure => Some(SynchronisedRegistryAcquireAccessError::VerificationFailure),
                SynchronisedRegistryCheckAccessResult::ListsDenied => Some(SynchronisedRegistryAcquireAccessError::ListsDenied),
                SynchronisedRegistryCheckAccessResult::ContainsResource | SynchronisedRegistryCheckAccessResult::MissingResource => None,
            }?;
            Some((index, error))
        }).collect::<Vec<_>>();
        let has_missing_resource = checks.iter().any(|check| matches!(check, SynchronisedRegistryCheckAccessResult::MissingResource))
            || (resource_access.is_empty() && registry.len() == 0);
        let can_acquire_access = match access {
            Access::Shared(count) => count > 0,
            Access::Unique => true,
            Access::Replace => false,
        };
        let should_acquire = duplicate_ids.is_empty() && denied.is_empty() && !has_missing_resource && can_acquire_access;
        let actual = registry.acquire_view::<AccessResult<ResourceView<'_, ResourceId, BruteStoredResource>>>(RegistryAcquireView {
            user_details: user_details.as_ref().map(|(id, password)| (id, password)),
            resource_access: resource_access.iter().map(|(id, password)| (id.clone(), password.as_ref())).collect(),
            access: access.clone(),
        });

        if should_acquire {
            let Ok(acquired) = actual else { panic!("Expected view acquisition to succeed") };
            let expected_ids = resource_access.iter().map(|(resource_id, _)| resource_id.clone()).collect::<HashSet<_>>();
            match acquired {
                AccessResult::Shared(ResourceView::Shared(view)) => {
                    let actual_ids = view.keys().cloned().collect::<HashSet<_>>();
                    assert_eq!(actual_ids, expected_ids);
                    for resource_id in &expected_ids {
                        assert!(view.get(resource_id).is_some());
                    }
                }
                AccessResult::Unique(ResourceView::Unique(view)) => {
                    let actual_ids = view.keys().cloned().collect::<HashSet<_>>();
                    assert_eq!(actual_ids, expected_ids);
                    for resource_id in &expected_ids {
                        assert!(view.get(resource_id).is_some());
                    }
                }
                _ => panic!("view result did not match the requested access mode"),
            }
            for (resource_id, _) in &resource_access {
                self.record_and_release_access(registry, resource_id, access.clone());
            }
        } else if !duplicate_ids.is_empty() {
            assert!(matches!(actual, Err(aion_state::prelude::SynchronisedRegistryAcquireViewError::DuplicateIds(indices)) if indices == duplicate_ids));
        } else if !denied.is_empty() {
            assert!(matches!(actual, Err(aion_state::prelude::SynchronisedRegistryAcquireViewError::Reception(errors)) if errors == denied));
        } else if has_missing_resource {
            assert!(matches!(actual, Err(aion_state::prelude::SynchronisedRegistryAcquireViewError::NotFound)));
        } else {
            assert!(matches!(actual, Err(aion_state::prelude::SynchronisedRegistryAcquireViewError::TriedAcquiring)),
                "unexpected view outcome for access {access:?}, checks {checks:?}, resources {resource_access:?}");
        }
    }

    #[cfg(not(feature = "stable_deref"))]
    fn test_reallocating_replacement(
        &mut self,
        registry: &Arc<TestRegistry>,
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        resource_id: ResourceId,
        resource: Option<Resource>,
        password: Option<Password>,
    ) {
        self.test_checked_replacement(registry, user_details, credentials_match, Access::Replace, resource_id, resource, password);
    }

    #[cfg(feature = "stable_deref")]
    fn test_reallocating_replacement(
        &mut self,
        registry: &Arc<TestRegistry>,
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        resource_id: ResourceId,
        resource: Option<Resource>,
        password: Option<Password>,
    ) {
        self.known_resource_ids.insert(resource_id.clone());
        let contains = self.resources.contains(&resource_id);
        assert_eq!(contains, registry.contains_resource(&RegistryContainsResource { resource_id: &resource_id }).ok());
        let resources_before = registry.len();
        let check = self.expected_check_access(
            user_details.as_ref().map(|(id, password)| (id, password)),
            credentials_match,
            &resource_id,
            &Access::Replace,
            password.as_ref(),
        );
        let authorized = matches!(check, SynchronisedRegistryCheckAccessResult::ContainsResource | SynchronisedRegistryCheckAccessResult::MissingResource);
        let resource_is_some = resource.is_some();
        let expected_resource = resource.clone();
        let expected_old_resource = self.resource_values.get(&resource_id).cloned();
        let actual = registry.reallocating_replace(RegistryReplacement {
            user_details: user_details.as_ref().map(|(id, password)| (id, password)),
            access: &Access::Replace,
            resource_id: resource_id.clone(),
            resource,
            password: password.as_ref(),
        });

        let succeeded = if !authorized {
            match check {
                SynchronisedRegistryCheckAccessResult::Err => assert!(matches!(actual, aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::AccessConflict)),
                SynchronisedRegistryCheckAccessResult::ReservationConflict => assert!(matches!(actual, aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::ReservationConflict)),
                SynchronisedRegistryCheckAccessResult::VerificationFailure => assert!(matches!(actual, aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::VerificationFailure)),
                SynchronisedRegistryCheckAccessResult::ListsDenied => assert!(matches!(actual, aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::ListsDenied)),
                SynchronisedRegistryCheckAccessResult::ContainsResource | SynchronisedRegistryCheckAccessResult::MissingResource => {
                    if !resource_is_some && !contains {
                        assert!(matches!(actual, aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::NoOp));
                    } else {
                        assert!(matches!(actual, aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::DeniedAccess));
                    }
                }
            }
            false
        } else if resource_is_some || contains {
            if contains {
                let aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::Found(previous) = &actual else {
                    panic!("replacing a stored stable value should return its previous value");
                };
                assert_eq!(Some(previous.clone()), expected_old_resource);
            } else {
                assert!(matches!(&actual, aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::NotFound));
            }
            if resource_is_some == contains {
                assert_eq!(resources_before, registry.len());
            } else if resource_is_some {
                assert_eq!(resources_before + 1, registry.len());
            } else {
                assert_eq!(resources_before - 1, registry.len());
            }
            true
        } else {
            assert!(matches!(actual, aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::NoOp));
            assert_eq!(resources_before, registry.len());
            false
        };

        if succeeded {
            if let Some(expected_resource) = expected_resource.as_ref() {
                let probe = registry.reallocating_replace(RegistryReplacement {
                    user_details: user_details.as_ref().map(|(id, password)| (id, password)),
                    access: &Access::Replace,
                    resource_id: resource_id.clone(),
                    resource: Some(expected_resource.clone()),
                    password: password.as_ref(),
                });
                let aion_state::prelude::SynchronisedRegistryReallocatingReplacementResult::Found(previous) = probe else {
                    panic!("replacing the stable stored value should return the previous value");
                };
                assert_eq!(previous, *expected_resource);
            }
            if let Some(expected_resource) = expected_resource {
                self.resources.insert(resource_id.clone());
                self.resource_values.insert(resource_id, expected_resource);
            } else {
                self.resources.remove(&resource_id);
                self.resource_values.remove(&resource_id);
            }
        }
        assert_eq!(self.resources.len(), registry.len());
    }
}
