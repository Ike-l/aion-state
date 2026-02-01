use crate::prelude::SingularRegistry;

pub mod singular_registry;

pub struct Registry<S, T, R> {
    sync: parking_lot::Mutex<()>,
    singular_registry: SingularRegistry<S, T, R>,
}

impl<S, T, R> Registry<S, T, R> {
    
}