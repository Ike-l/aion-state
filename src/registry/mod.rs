use crate::prelude::{SingularRegistry, sync::Mutex};

pub mod singular_registry;

pub struct Registry<S, RS, AS, OS, PS> {
    sync: Mutex<()>,
    singular_registry: SingularRegistry<S, RS, AS, OS, PS>,
}
