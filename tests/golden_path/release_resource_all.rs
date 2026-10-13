use aion_state::prelude::{RegistryIsOwned, RegistryOwn, RegistryRegister, RegistryReleaseResourceAll, SynchronisedRegistryReleaseResourceAllResult, SynchronisedRegistryReleaseResourceResult};

use crate::default::prelude::*;
use crate::create_registry;

fn can_release_resource_all() {
    let registry = create_registry(None);

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());

    let resource_id_1 = ResourceId::new_type::<i32>();
    let result = registry.own(RegistryOwn {
        id: id.clone(),
        password: &password,
        resource_id: resource_id_1.clone()
    });

    assert!(result.ok());

    let resource_id_2 = ResourceId::new_type::<bool>();
    let result = registry.own(RegistryOwn {
        id: id.clone(),
        password: &password,
        resource_id: resource_id_2.clone()
    });

    assert!(result.ok());

    let guest = ReserverId::new("guest");
    let guest_password = Password::from(2);
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: guest_password.clone() }).ok());
    assert!(matches!(registry.release_resource_all(RegistryReleaseResourceAll {
        id: &id, password: &Password::from(2), inputs: vec![&resource_id_1, &resource_id_2],
    }), SynchronisedRegistryReleaseResourceAllResult::VerificationFailure));
    let denied = registry.release_resource_all(RegistryReleaseResourceAll {
        id: &guest, password: &guest_password, inputs: vec![&resource_id_1, &resource_id_2],
    });
    assert!(matches!(denied,
        SynchronisedRegistryReleaseResourceAllResult::All(results)
            if results.len() == 2 && results.iter().all(|result| matches!(result, SynchronisedRegistryReleaseResourceResult::OwnershipDenied))
    ));
    assert!(registry.is_owned(&RegistryIsOwned { resource_id: &resource_id_1 }));
    assert!(registry.is_owned(&RegistryIsOwned { resource_id: &resource_id_2 }));
    assert!(matches!(registry.release_resource_all(RegistryReleaseResourceAll {
        id: &id, password: &password, inputs: vec![],
    }), SynchronisedRegistryReleaseResourceAllResult::All(results) if results.is_empty()));

    let result = registry.release_resource_all(RegistryReleaseResourceAll {
        id: &id,
        password: &password,
        inputs: vec![&resource_id_1, &resource_id_2],
    });

    match result {
        SynchronisedRegistryReleaseResourceAllResult::All(registry_release_resource_results) => {
            assert_eq!(registry_release_resource_results.len(), 2);
            for result in registry_release_resource_results {
                assert!(result.ok());
            }
        },
        SynchronisedRegistryReleaseResourceAllResult::VerificationFailure => panic!("Expected Success"),
    }
    assert!(!registry.is_owned(&RegistryIsOwned { resource_id: &resource_id_1 }));
    assert!(!registry.is_owned(&RegistryIsOwned { resource_id: &resource_id_2 }));
    assert!(matches!(registry.release_resource_all(RegistryReleaseResourceAll {
        id: &id, password: &password, inputs: vec![&resource_id_1, &resource_id_2],
    }), SynchronisedRegistryReleaseResourceAllResult::All(results)
        if results.len() == 2 && results.iter().all(|result| matches!(result, SynchronisedRegistryReleaseResourceResult::OwnershipDenied))));
}

#[test]
fn can_release_resource_all_normal() {
    can_release_resource_all();
}
