pub mod access_map;

pub struct StoredAccesses<A> {
    access_map: parking_lot::RwLock<A>
}