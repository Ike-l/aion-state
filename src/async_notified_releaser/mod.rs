use crate::prelude::{AsyncNotifier, AsyncReleaser};

pub mod async_future_acquire_released_access;

pub trait AsyncNotifiedReleaser<Value, AccessInput, Error>: AsyncNotifier<Value, AccessInput = AccessInput, Error = Error> + AsyncReleaser<Value, AccessInput = AccessInput, Error = Error> {}
