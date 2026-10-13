use aion_state::prelude::Registry;

pub mod default;
mod golden_path;
mod acquire_access;
mod serde;
mod brute;
mod feature_tests;

use default::prelude::*;

use tracing_subscriber::{EnvFilter, fmt};
use std::sync::Once;
#[cfg(feature = "async")]
use std::{future::Future, sync::Arc, task::{Context, Poll, Wake, Waker}};

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

#[cfg(feature = "async")]
struct AsyncNoopWake;

#[cfg(feature = "async")]
impl Wake for AsyncNoopWake {
    fn wake(self: Arc<Self>) {}
}

#[cfg(feature = "async")]
pub(crate) fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(AsyncNoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

// cargo test --features default-implementation -- --no-capture

pub type TestRegistry = Registry<RegistryStorage<ResourceId, StoredResource>, ReservationStorage<ReserverId, AccessStorage<ResourceId, Access>>, AccessStorage<ResourceId, Access>, CredentialStorage<ReserverId, Password>, WhitelistStorage<ResourceId, Access>, BlacklistStorage<ResourceId, Access, Password>, ControlStorage<ReserverId, ResourceId>>;

#[cfg(feature = "stable_deref")]
pub type BruteStoredResource = Box<Resource>;
#[cfg(not(feature = "stable_deref"))]
pub type BruteStoredResource = StoredResource;

pub type BruteRegistry = Registry<RegistryStorage<ResourceId, BruteStoredResource>, ReservationStorage<ReserverId, AccessStorage<ResourceId, Access>>, AccessStorage<ResourceId, Access>, CredentialStorage<ReserverId, Password>, WhitelistStorage<ResourceId, Access>, BlacklistStorage<ResourceId, Access, Password>, ControlStorage<ReserverId, ResourceId>>;

pub fn create_registry(capacity: Option<usize>) -> TestRegistry {
    init_tracing();

    let registry_storage = RegistryStorage::new(capacity.unwrap_or(100));

    Registry::new(registry_storage, Default::default())
}

pub fn create_brute_registry(capacity: Option<usize>) -> BruteRegistry {
    init_tracing();

    let registry_storage = RegistryStorage::new(capacity.unwrap_or(100));

    Registry::new(registry_storage, Default::default())
}

#[cfg(feature = "sync")]
#[test]
fn synchronized_registry_registers_concurrently_without_losing_users() {
    const WORKERS: usize = 8;

    let registry = std::sync::Arc::new(create_registry(None));
    let start = std::sync::Arc::new(std::sync::Barrier::new(WORKERS + 1));
    let handles = (0..WORKERS).map(|worker| {
        let registry = registry.clone();
        let start = start.clone();
        std::thread::spawn(move || {
            let id = ReserverId::new(format!("concurrent-user-{worker}"));
            let password = Password::new(worker as u64);
            start.wait();
            assert!(registry.register(aion_state::prelude::RegistryRegister {
                id: id.clone(),
                password,
            }).ok());
            id
        })
    }).collect::<Vec<_>>();

    start.wait();
    let ids = handles.into_iter().map(|handle| handle.join().unwrap()).collect::<Vec<_>>();
    let registered = registry.registered();
    assert_eq!(WORKERS, registered.len());
    for id in ids {
        assert!(registered.contains(&id), "concurrent registration lost {id:?}");
    }
}
