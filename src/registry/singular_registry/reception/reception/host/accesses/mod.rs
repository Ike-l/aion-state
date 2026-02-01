use std::fmt::Debug;

use tracing::{field, span};

use crate::prelude::{AccessPermission, AccessStorage, Accessor, FUNCTION_LEVEL, PermitsAccessInput, RecordAccessInput, RecordAccessResult, RemoveAccessResult};

pub mod accesses_input;
pub mod accesses_result;
pub mod access_storage;

#[derive(Default)]
pub struct Accesses<AS> {
    access_storage: AS
}

impl<AS: AccessStorage> Accesses<AS> 
    where AS::Value: Accessor + Debug
{
    pub fn permits_access(
        &self,
        PermitsAccessInput {
            access_key, access
        }: PermitsAccessInput<'_, AS::Key, AS::Value>
    ) -> AccessPermission {
        let span = span!(FUNCTION_LEVEL, "Accesses Permits Access", current_access = field::Empty);
        let _enter = span.enter();

        if let Some(current_access) = self.access_storage.get(access_key) {
            span.record("current_access", format!("{current_access:?}"));
            AccessPermission::Ok(current_access.can_access(access))
        } else {
            AccessPermission::NoCurrentAccess
        }
    }

    pub fn record_access(
        &mut self,
        RecordAccessInput {
            access_key, access
        }: RecordAccessInput<AS::Key, AS::Value>
    ) -> RecordAccessResult {
        let span = span!(FUNCTION_LEVEL, "Accesses Record Access", current_access = field::Empty);
        let _enter = span.enter();

        if let Some(current_access) = self.access_storage.get_mut(&access_key) {
            span.record("current_access", format!("{current_access:?}"));
            current_access.merge(access);
            RecordAccessResult::Merged
        } else {
            self.access_storage.insert(access_key, access);
            RecordAccessResult::Inserted
        }
    }

    pub fn remove_access(
        &mut self
    ) -> RemoveAccessResult {
        todo!()
    }
}