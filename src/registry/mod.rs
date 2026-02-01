use crate::prelude::SingularRegistry;

pub mod singular_registry;

pub struct Registry<S, RS, AS, OS, PS> {
    sync: parking_lot::Mutex<()>,
    singular_registry: SingularRegistry<S, RS, AS, OS, PS>,
}
