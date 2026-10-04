use crate::prelude::{AsyncNotifier, AsyncReleaser};

pub mod async_future_acquire_released_access;

pub trait AsyncNotifiedReleaser<'a, Value, AccessInput, Error>: AsyncNotifier<'a, Value, AccessInput = AccessInput, Error = Error> + AsyncReleaser<'a, Value, AccessInput = AccessInput, Error = Error> {}
