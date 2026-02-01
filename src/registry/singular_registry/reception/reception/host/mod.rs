use crate::prelude::{Reservations, Accesses};

pub mod reservations;
pub mod accesses;

pub struct Host<RMap, AMap> {
    reservations: Reservations<RMap>,
    stored_accesses: Accesses<AMap>,
}