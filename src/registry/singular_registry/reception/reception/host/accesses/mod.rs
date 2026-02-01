use tracing::{field, span};

use crate::prelude::{AccessPermission, AccessStorage, FUNCTION_LEVEL, PermitsAccessInput, RecordAccessInput};

pub mod accesses_input;
pub mod accesses_result;
pub mod access_storage;

#[derive(Default)]
pub struct Accesses<AS> {
    access_map: AS
}

impl<AS: AccessStorage> Accesses<AS> {
    pub fn permits_access(
        &self,
        PermitsAccessInput {
            access_key, access
        }: PermitsAccessInput<'_, AS::Key, AS::Value>
    ) -> AccessPermission {
        let span = span!(FUNCTION_LEVEL, "Accesses Permits Access", current_access = field::Empty);
        let _enter = span.enter();

        todo!()
    }

    pub fn record_access(
        &mut self,
        RecordAccessInput {
            access_key, access
        }: RecordAccessInput<AS::Key, AS::Value>
    ) {
        let span = span!(FUNCTION_LEVEL, "Accesses Record Access");
        let _enter = span.enter();

        todo!()
    }
}