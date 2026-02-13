use std::fmt::Debug;

use tracing::{field, span};

use crate::prelude::{AccessPermission, AccessStorage, Accessor, FUNCTION_LEVEL, PermitsAccessInput, RecordAccessInput, RecordAccessResult, RemoveAccessInput, RemoveAccessResult};

pub mod accesses_input;
pub mod accesses_result;
pub mod access_storage;

#[derive(Default)]
pub struct Accesses<AS> {
    access_storage: AS
}

impl<AS: AccessStorage> Accesses<AS> 
    where AS::Access: Accessor + Debug
{
    pub fn permits_access(
        &self,
        PermitsAccessInput {
            access_id, access
        }: PermitsAccessInput<'_, AS::AccessId, AS::Access>
    ) -> AccessPermission {
        let span = span!(FUNCTION_LEVEL, "Accesses Permits Access", current_access = field::Empty);
        let _enter = span.enter();

        if let Some(current_access) = self.access_storage.get(access_id) {
            span.record("current_access", format!("{current_access:?}"));
            AccessPermission::Ok(current_access.can_access(access))
        } else {
            AccessPermission::NoCurrentAccess
        }
    }

    pub fn record_access(
        &mut self,
        RecordAccessInput {
            access_id, access
        }: RecordAccessInput<AS::AccessId, AS::Access>
    ) -> RecordAccessResult {
        let span = span!(FUNCTION_LEVEL, "Accesses Record Access", current_access = field::Empty);
        let _enter = span.enter();

        if let Some(current_access) = self.access_storage.get_mut(&access_id) {
            span.record("current_access", format!("{current_access:?}"));
            current_access.merge(access);
            RecordAccessResult::Merged
        } else {
            self.access_storage.insert(access_id, access);
            RecordAccessResult::Inserted
        }
    }

    pub fn release_access(
        &mut self,
        RemoveAccessInput {
            access_id, access
        }: RemoveAccessInput<AS::AccessId, AS::Access>
    ) -> RemoveAccessResult {
        let span = span!(FUNCTION_LEVEL, "Accesses Release Access", current_access = field::Empty);
        let _enter = span.enter();

        if let Some(current_access) = self.access_storage.get_mut(access_id) {
            span.record("current_access", format!("{current_access:?}"));

            current_access.release(access);
            RemoveAccessResult::Split
        } else {
            RemoveAccessResult::NoCurrentAccess
        }
    }
}