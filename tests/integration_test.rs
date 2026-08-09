use aion_state::prelude::Registry;

pub mod default;
mod golden_path;
mod acquire_access;
mod serde;
mod brute;

use default::prelude::*;

use tracing_subscriber::{EnvFilter, fmt};
use std::sync::Once;

static INIT: Once = Once::new();

fn init_tracing() {
    INIT.call_once(|| {
        fmt()
            .with_ansi(false)
            .compact()
            // .pretty()
            .with_env_filter(EnvFilter::new("info,aion_state=debug"))
            // .with_max_level(tracing::Level::TRACE)
            // .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::EXIT)
            .with_target(false)
            .with_test_writer()           
            .init();
    });
}

// cargo test --features default-implementation -- --no-capture

pub type TestRegistry = Registry<RegistryStorage<ResourceId, StoredResource>, ReservationStorage<ReserverId, AccessStorage<ResourceId, Access>>, AccessStorage<ResourceId, Access>, CredentialStorage<ReserverId, Password>, WhitelistStorage<ResourceId, Access>, BlacklistStorage<ResourceId, Access, Password>, ControlStorage<ReserverId, ResourceId>>;

pub fn create_registry(capacity: Option<usize>) -> TestRegistry {
    init_tracing();

    let registry_storage = RegistryStorage::new(capacity.unwrap_or(100));

    Registry::new(registry_storage)
}
