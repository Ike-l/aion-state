use aion_state::prelude::{RegistryIsOwned, RegistryOwn, RegistryRegister, RegistryReleaseResource, SynchronisedRegistryReleaseResourceResult};

use crate::default::prelude::*;
use crate::create_registry;

fn can_release_resource() {
    let registry = create_registry(None);

    let id = ReserverId::new("foo");
    let password = Password::from(1);

    let result = registry.register(RegistryRegister { id: id.clone(), password: password.clone() });

    assert!(result.ok());

    let resource_id = ResourceId::new_type::<i32>();
    let result = registry.own(RegistryOwn {
        id: id.clone(),
        password: &password,
        resource_id: resource_id.clone()
    });

    assert!(result.ok());

    let guest = ReserverId::new("guest");
    let guest_password = Password::from(2);
    assert!(registry.register(RegistryRegister { id: guest.clone(), password: guest_password.clone() }).ok());
    assert!(matches!(registry.release_resource(&RegistryReleaseResource {
        id: &guest, password: &guest_password, resource_id: &resource_id,
    }), SynchronisedRegistryReleaseResourceResult::OwnershipDenied));
    assert!(matches!(registry.release_resource(&RegistryReleaseResource {
        id: &id, password: &Password::from(2), resource_id: &resource_id,
    }), SynchronisedRegistryReleaseResourceResult::VerificationFailure));
    assert!(registry.is_owned(&RegistryIsOwned { resource_id: &resource_id }));

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(std::io::sink)
        .finish();
    let result = tracing::subscriber::with_default(subscriber, || {
        assert!(tracing::enabled!(tracing::Level::TRACE));
        registry.release_resource(&RegistryReleaseResource {
            id: &id,
            password: &password,
            resource_id: &resource_id
        })
    });

    assert!(result.ok());
    assert!(!registry.is_owned(&RegistryIsOwned { resource_id: &resource_id }));
    assert!(matches!(registry.release_resource(&RegistryReleaseResource {
        id: &id, password: &password, resource_id: &resource_id,
    }), SynchronisedRegistryReleaseResourceResult::OwnershipDenied));
}

#[test]
fn can_release_resource_normal() {
    can_release_resource();
}
