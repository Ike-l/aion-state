#![cfg(feature = "default-implementation")]

use aion_state::prelude::{Registry, RegistryReleaseResource};

use aion_state::default::prelude::*;

pub fn create_registry() -> Registry<
    RegistryStorage<ResourceId, StoredResource>,
    ReservationStorage<ReserverId, AccessStorage<ResourceId, Access>>,
    AccessStorage<ResourceId, Access>,
    CredentialStorage<ReserverId, Password>,
    WhitelistStorage<ResourceId, Access>,
    BlacklistStorage<ResourceId, Access, Password>,
    ControlStorage<ReserverId, ResourceId>
> {
    Registry::default()
}


pub fn a() {
    let registry = create_registry();

    let id = ReserverId::new("foo");
    let password = Password::from(1);
    let resource_id = ResourceId::new_label("Bar");

    let input = RegistryReleaseResource {
        id: &id, password: &password, resource_id: &resource_id
    };

    let result = registry.release_resource(&input);

    match result {
        aion_state::prelude::RegistryReleaseResourceResult::Ok => todo!(),
        aion_state::prelude::RegistryReleaseResourceResult::Err => todo!(),
        aion_state::prelude::RegistryReleaseResourceResult::OwnershipDenied => todo!(),
        aion_state::prelude::RegistryReleaseResourceResult::VerificationFailure => todo!(),
    }
}