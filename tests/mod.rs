#![cfg(feature = "default-implementation")]

use aion_state::prelude::{OwnerRegisterResult, ReceptionRegisterResult, Registry, RegistryRegister, RegistryRegisterResult, AuthenticateRegistrationResult};

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

    let id = ReserverId::new("Foo");
    let password = Password::from(1);

    let input = RegistryRegister {
        id, password
    };
    
    let RegistryRegisterResult::Reception(
        ReceptionRegisterResult::Owner(
            OwnerRegisterResult::Authenticator(
                AuthenticateRegistrationResult::Registration(register_result)
            )
        )
    ) = registry.register(input);

    if register_result {
        
    }
}