use tracing::{field, span};

use crate::prelude::{AccessPermission, FUNCTION_LEVEL, Storage};

pub mod access_map_input;
pub mod access_map_result;

pub struct AccessMap<S> {
    accesses: S
}

impl<S: Storage> AccessMap<S> {
    pub fn permits_access(
        &self,
        access_key: &S::Key,
        access: &S::Value,
    ) -> AccessPermission {
        let span = span!(FUNCTION_LEVEL, "AccessMap Permits Access", current_access = field::Empty);
        let _enter = span.enter();

        // if let Some(current_access) = self.accesses.get(access_key)
        todo!()
    }
}