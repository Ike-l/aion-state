use crate::prelude::{SingularRegistry, sync::Mutex};

pub mod singular_registry;

/// Separate Sync bc the point is to not use RAII, 
/// removing the sync and making the functions take `&mut self` would require some form of RAII in mt situations
pub struct Registry<S, RS, AS, OS, PS, LS, OSS> {
    sync: Mutex<()>,
    singular_registry: SingularRegistry<S, RS, AS, OS, PS, LS, OSS>,
}
