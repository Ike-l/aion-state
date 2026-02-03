use tracing::span;

use crate::prelude::{Accessor, FUNCTION_LEVEL, ManualRegistryAccessInput, ManualRegistryAccessResult, ManualRegistryReplacementInput, ManualRegistryReplacementResult, RegistryStorage, StableAddress};

pub mod registry_storage;
pub mod manual_registry_input;
pub mod manual_registry_result;
pub mod stable_address;

pub struct ManualRegistry<S> {
    storage: S,
}

impl<
    S: RegistryStorage,
> ManualRegistry<S> {
    pub fn acquire_access<Access: Accessor<StoredValue = S::Value>>(
        &self, 
        ManualRegistryAccessInput {
            key, access
        }: ManualRegistryAccessInput<'_, Access, S::Key>
    ) -> ManualRegistryAccessResult<Access::AccessResult<'_>> {
        let span = span!(FUNCTION_LEVEL, "Manual Acquire Access");
        let _enter = span.enter();

        match self.storage.get(key) {
            Some(value) => ManualRegistryAccessResult::Found(access.acquire(value)),
            None => ManualRegistryAccessResult::NotFound,
        }
    }

    // private until i can figure out a way to detect reallocations before they happen
    /// Safety:
    /// Insert won't invalidate concurrent access
    /// ^ i.e do not "replace" a borrowed item
    /// ^ i.e do not insert if it could reallocate container and invalidate concurrent accesses
    unsafe fn replace<Access: Accessor<StoredValue = S::Value>>(
        &mut self,
        ManualRegistryReplacementInput {
            access, key, value
        }: ManualRegistryReplacementInput<'_, Access, S::Key, Access::Value>
    ) -> ManualRegistryReplacementResult<Access::StoredValue> {
        let span = span!(FUNCTION_LEVEL, "Manual Unsafe Replacement");
        let _enter = span.enter();

        let old_resource = match (
            value,
            self.storage.contains_key(&key),
            access.can_insert(),
            access.can_remove(),
        ) {
            // if contains resource (so remove) but denied removal
            (_, true, _, false) |
            // if input contains resource (so insert) but denied insert
            (Some(_), _, false, _) => return ManualRegistryReplacementResult::DeniedAccess,

            // if does not contain resource and not input 
            (None, false, _, _) => return ManualRegistryReplacementResult::NoOp,

            // removal and allowed remove
            (None, true, _, true) => self.storage.remove(&key),
            
            // replacement and allowed insert & remove
            (Some(new_value), true, true, true) |

            // insert without replacement and allowed insert
            (Some(new_value), false, true, _) => {
                let new_stored_value = access.insert(new_value);
                self.storage.insert(key, new_stored_value)
            },            
        };

        match old_resource {
            Some(found) => ManualRegistryReplacementResult::Found(access.remove(found)),
            None => todo!(),
        }
    }

    // safer bc trade reallocation requirement for the don't reference the stable address stored vaslue itself
    /// Safety:
    /// Access cannot be used to 'acquire' a reference to StableAddress itself
    /// Insert won't invalidate concurrent access
    /// ^ i.e do not replace a borrowed item
    pub unsafe fn safer_replace<Access: Accessor<StoredValue = S::Value>>(
        &mut self,
        manual_registry_replacement_input: ManualRegistryReplacementInput<'_, Access, S::Key, Access::Value>
    ) -> ManualRegistryReplacementResult<Access::StoredValue> 
        where Access::StoredValue: StableAddress
    {
        let span = span!(FUNCTION_LEVEL, "Manual Safe Replacement");
        let _enter = span.enter();

        // Safety:
        // if a container of StableAddress reallocates, pointers are still valid
        unsafe { self.replace(manual_registry_replacement_input) }
    }

    pub fn contains_key(
        &self,
        key: &S::Key
    ) -> bool {
        self.storage.contains_key(key)
    }
}
