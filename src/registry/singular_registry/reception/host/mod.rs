use crate::prelude::{Reservations, StoredAccesses};

pub mod reservations;
pub mod stored_accesses;

pub struct Host<S, R> {
    reservations: Reservations<S, R>,
    stored_accesses: StoredAccesses<S>,
}