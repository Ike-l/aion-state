use aion_state::prelude::Accessor;

use crate::default::prelude::*;

#[test]
fn default_accessor_modes_define_compatibility_and_mutation() {
    assert!(Access::Shared(1).accepts_incoming(&Access::Shared(2)));
    assert!(!Access::Shared(1).accepts_incoming(&Access::Unique));
    assert!(!Access::Unique.accepts_incoming(&Access::Shared(1)));
    assert!(Access::Shared(0).accepts_incoming(&Access::Unique));
    assert!(!Access::Shared(1).accepts_incoming(&Access::Replace));
    assert!(!Access::Unique.accepts_incoming(&Access::Replace));
    assert!(Access::Replace.accepts_incoming(&Access::Shared(1)));
    assert!(Access::Replace.accepts_incoming(&Access::Unique));
    assert!(Access::Replace.accepts_incoming(&Access::Replace));

    assert!(Access::Replace.can_insert_resource());
    assert!(Access::Replace.can_remove_resource());
    assert!(!Access::Shared(1).can_insert_resource());
    assert!(!Access::Unique.can_remove_resource());
}

#[test]
fn default_accessor_merges_and_releases_shared_counts() {
    let mut access = Access::Shared(1);
    access.merge(Access::Shared(2));
    assert_eq!(Access::Shared(3), access);

    access.release(&Access::Shared(1));
    assert_eq!(Access::Shared(2), access);

    access.merge(Access::Shared(0));
    assert_eq!(Access::Shared(2), access);

    let mut unique = Access::Unique;
    unique.release(&Access::Unique);
    assert_eq!(Access::Shared(0), unique);
}

#[test]
fn default_accessor_builds_the_requested_reference_result() {
    use crate::default::storages::registry_storage::ResourceWrapper;

    let mut resource = Resource::new("shared".to_string());
    let shared = Access::Shared(1).acquire::<_, AccessResult<ResourceWrapper<'_, Resource>>>(
        ResourceWrapper::new(&mut resource),
    );
    assert!(matches!(shared, Some(AccessResult::Shared(ResourceWrapper::Shared(_)))));

    let unique = Access::Unique.acquire::<_, AccessResult<ResourceWrapper<'_, Resource>>>(
        ResourceWrapper::new(&mut resource),
    );
    assert!(matches!(unique, Some(AccessResult::Unique(ResourceWrapper::Unique(_)))));

    assert!(Access::Shared(0).acquire::<_, AccessResult<ResourceWrapper<'_, Resource>>>(
        ResourceWrapper::new(&mut resource),
    ).is_none());
    assert!(Access::Replace.acquire::<_, AccessResult<ResourceWrapper<'_, Resource>>>(
        ResourceWrapper::new(&mut resource),
    ).is_none());
}
