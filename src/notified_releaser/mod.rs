use crate::prelude::{Notifier, Releaser};

pub mod future_acquire_released_access;

pub trait NotifiedReleaser<'a, Value, AccessInput, Error>: Notifier<'a, Value, AccessInput = AccessInput, Error = Error> + Releaser<'a, Value, AccessInput = AccessInput, Error = Error> {}
