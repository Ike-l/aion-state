use std::sync::Arc;

use crate::{TestRegistry, brute::command::Command};

pub struct PropertyManager {}

impl PropertyManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn test(&self, registry: &Arc<TestRegistry>, command: Command) {
        todo!()
    }
}