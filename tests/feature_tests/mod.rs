#[cfg(feature = "notifier")]
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
#[cfg(feature = "notifier")]
use std::task::{Wake, Waker};

mod accessor;
mod storage_contracts;
mod result_conversions;

#[cfg(feature = "releaser")]
mod releaser;

#[cfg(feature = "notifier")]
mod notifier;

#[cfg(feature = "async")]
mod asynchronous;
#[cfg(all(feature = "async", feature = "notifier"))]
mod async_pending;

#[cfg(all(feature = "async", feature = "notifier"))]
mod async_notifier;

#[cfg(all(feature = "async", feature = "notifier", feature = "releaser"))]
mod async_notified_releaser;

#[cfg(all(feature = "notifier", feature = "releaser"))]
mod notified_releaser;

#[cfg(feature = "stable_deref")]
mod stable_deref;

#[cfg(feature = "notifier")]
pub(super) struct WakeCount(AtomicUsize);

#[cfg(feature = "notifier")]
impl WakeCount {
    pub(super) fn count(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}

#[cfg(feature = "notifier")]
impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(feature = "notifier")]
pub(super) fn counting_waker() -> (Waker, Arc<WakeCount>) {
    let counter = Arc::new(WakeCount(AtomicUsize::new(0)));
    (Waker::from(counter.clone()), counter)
}
