use crate::prelude::Reception;

pub mod reception;

pub struct CoordinatedReception<RS, AS, OS, PS> {
    reception: parking_lot::RwLock<Reception<RS, AS, OS, PS>>
}
