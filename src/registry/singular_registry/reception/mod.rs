use crate::prelude::Reception;

pub mod reception;

pub struct CoordinatedReception<RS, AS, OS> {
    reception: parking_lot::RwLock<Reception<RS, AS, OS>>
}
