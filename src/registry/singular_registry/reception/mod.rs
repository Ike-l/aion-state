use crate::prelude::{Reception, sync::RwLock};

pub mod reception;

pub struct CoordinatedReception<RS, AS, OS, PS> {
    reception: RwLock<Reception<RS, AS, OS, PS>>
}
