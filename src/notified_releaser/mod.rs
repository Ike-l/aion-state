use crate::prelude::{Notifier, Releaser};

pub mod future_acquire_released_access;

pub trait NotifiedReleaser<Value, AccessInput, Error>: Notifier<Value, AccessInput = AccessInput, Error = Error> + Releaser<Value, AccessInput = AccessInput, Error = Error> {}
