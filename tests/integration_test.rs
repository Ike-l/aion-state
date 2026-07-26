#![cfg(feature = "default-implementation")]

use aion_state::prelude::Registry;

use aion_state::default::prelude::*;

mod golden_path;
mod brute;

use tracing_subscriber::fmt;
use std::sync::Once;

static INIT: Once = Once::new();

fn init_tracing() {
    INIT.call_once(|| {
        fmt()
            // .with_ansi(false)
            .compact()
            // .pretty()
            // .with_env_filter(EnvFilter::new("info,aion_reactor=debug"))
            .with_max_level(tracing::Level::TRACE)
            // .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::EXIT)
            .with_target(false)
            .with_test_writer()           
            .init();
    });
}

// cargo test --features default-implementation -- --no-capture

pub type TestRegistry = Registry<RegistryStorage<ResourceId, Box<Resource>>, ReservationStorage<ReserverId, AccessStorage<ResourceId, Access>>, AccessStorage<ResourceId, Access>, CredentialStorage<ReserverId, Password>, WhitelistStorage<ResourceId, Access>, BlacklistStorage<ResourceId, Access, Password>, ControlStorage<ReserverId, ResourceId>>;

pub fn create_registry() -> TestRegistry {
    init_tracing();

    Registry::default()
}
