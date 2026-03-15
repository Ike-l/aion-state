use std::fmt::Debug;

use tracing::{field, span};

use crate::prelude::{AccessesCheckAccessResult, AccessStorage, Accessor, FUNCTION_LEVEL, AccessesCheckAccess, AccessesRecordAccess, AccessesRecordAccessResult, AccessesRelease, AccessesReleaseResult};

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
    pub fn check_access(
        &self,
        AccessesCheckAccess {
            access_id, access
        }: AccessesCheckAccess<'_, AS::ValueId, AS::Access>
    ) -> AccessesCheckAccessResult {
        let span = span!(FUNCTION_LEVEL, "Accesses Permits Access", current_access = field::Empty);
        let _enter = span.enter();

        if let Some(current_access) = self.access_storage.get(access_id) {
            span.record("current_access", format!("{current_access:?}"));
            AccessesCheckAccessResult::Ok(current_access.accepts_incoming(access))
        } else {
            AccessesCheckAccessResult::NoCurrentAccess
        }
    }

    /// Records the incoming access by using <Accessor>::merge or <Accessor>::insert if no current access exists
    /// 
    /// Does not check if `Accessor` is ok with the incoming access
    /// Use in conjunction with `permits_access` 
    pub fn record_access(
        &mut self,
        AccessesRecordAccess {
            access_id, access
        }: AccessesRecordAccess<AS::ValueId, AS::Access>
    ) -> AccessesRecordAccessResult {
        let span = span!(FUNCTION_LEVEL, "Accesses Record Access", current_access = field::Empty);
        let _enter = span.enter();

        if let Some(current_access) = self.access_storage.get_mut(&access_id) {
            span.record("current_access", format!("{current_access:?}"));
            current_access.merge(access);
            AccessesRecordAccessResult::Merged
        } else {
            self.access_storage.insert(access_id, access);
            AccessesRecordAccessResult::Inserted
        }
    }

    /// Releases the incoming access by either splitting from a current access or does nothing if there is no current access
    /// 
    /// Does not check if `Accessor` is ok with the incoming access 
    pub fn release(
        &mut self,
        AccessesRelease {
            access_id, access
        }: AccessesRelease<AS::ValueId, AS::Access>
    ) -> AccessesReleaseResult {
        let span = span!(FUNCTION_LEVEL, "Accesses Release Access", current_access = field::Empty);
        let _enter = span.enter();

        if let Some(current_access) = self.access_storage.get_mut(access_id) {
            span.record("current_access", format!("{current_access:?}"));

            current_access.release(access);
            AccessesReleaseResult::Split
        } else {
            AccessesReleaseResult::NoCurrentAccess
        }
    }
}