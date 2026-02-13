use std::fmt::Debug;

use tracing::{field, span};

use crate::prelude::{AccessPermission, AccessStorage, Accessor, FUNCTION_LEVEL, PermitsAccessInput, RecordAccessInput, RecordAccessResult, RemoveAccessInput, RemoveAccessResult};

pub mod accesses_input;
pub mod accesses_result;
pub mod access_storage;

/// Wraps `AccessStorage` with `Accessor`
#[derive(Default)]
pub struct Accesses<AS> {
    access_storage: AS
}

impl<AS: AccessStorage> Accesses<AS> 
    where AS::Access: Accessor + Debug
{
    /// Permits access if there are no current accesses
    /// 
    /// Else the current access accepts the incoming access
    pub fn permits_access(
        &self,
        PermitsAccessInput {
            access_id, access
        }: PermitsAccessInput<'_, AS::ValueId, AS::Access>
    ) -> AccessPermission {
        let span = span!(FUNCTION_LEVEL, "Accesses Permits Access", current_access = field::Empty);
        let _enter = span.enter();

        if let Some(current_access) = self.access_storage.get(access_id) {
            span.record("current_access", format!("{current_access:?}"));
            AccessPermission::Ok(current_access.accepts_incoming(access))
        } else {
            AccessPermission::NoCurrentAccess
        }
    }

    /// Records the incoming access by using <Accessor>::merge or <Accessor>::insert if no current access exists
    /// 
    /// Does not check if `Accessor` is ok with the incoming access
    /// Use in conjunction with `permits_access` 
    pub fn record_access(
        &mut self,
        RecordAccessInput {
            access_id, access
        }: RecordAccessInput<AS::ValueId, AS::Access>
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

    /// Releases the incoming access by either splitting from a current access or does nothing if there is no current access
    /// 
    /// Does not check if `Accessor` is ok with the incoming access 
    pub fn release_access(
        &mut self,
        RemoveAccessInput {
            access_id, access
        }: RemoveAccessInput<AS::ValueId, AS::Access>
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