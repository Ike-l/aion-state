use aion_state::prelude::{RegistryOwn, RegistryRegister, RegistryReleaseResourceAll, RegistryReleaseResourceAllResult};

use crate::default::prelude::*;
use crate::create_registry;

fn can_release_resource_all() {
    let registry = create_registry();

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

    let result = registry.release_resource_all(RegistryReleaseResourceAll {
        id: &id,
        password: &password,
        inputs: vec![&resource_id_1, &resource_id_2],
    });

    match result {
        RegistryReleaseResourceAllResult::All(registry_release_resource_results) => {
            for result in registry_release_resource_results {
                assert!(result.ok());
            }
        },
        RegistryReleaseResourceAllResult::VerificationFailure => panic!("Expected Success"),
    }
    // assert!(result.ok())
}

#[test]
fn can_release_resource_all_normal() {
    can_release_resource_all();
}
