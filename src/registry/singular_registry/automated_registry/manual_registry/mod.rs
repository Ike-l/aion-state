use tracing::span;

use crate::prelude::{Accessor, FUNCTION_LEVEL, ManualRegistryAccessInput, ManualRegistryAccessResult, ManualRegistryReplacementInput, ManualRegistryReplacementResult, RegistryStorage};

pub mod registry_storage;
pub mod manual_registry_input;
pub mod manual_registry_result;

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
    ) -> ManualRegistryAccessResult<Access::AccessResult<'_, Access::Value>> {
        let span = span!(FUNCTION_LEVEL, "Manual Acquire Access");
        let _enter = span.enter();

        match self.storage.get(key) {
            Some(value) => ManualRegistryAccessResult::Found(access.acquire(value)),
            None => ManualRegistryAccessResult::NotFound,
        }
    }

    pub fn reallocates_on_next_new_insert(&self) -> bool {
        self.storage.reallocates_on_next_new_insert()
    }

    /// Safety:
    /// Ensure the insert will not invalidate any concurrent accesses.
    /// i.e If an access exists, do not reallocate the accessed memory
    /// can use `reallocates_on_next_new_insert` & tracked accesses
    pub unsafe fn replace<Access: Accessor<StoredValue = S::Value>>(
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

    pub fn contains_key(
        &self,
        key: &S::Key
    ) -> bool {
        self.storage.contains_key(key)
    }
}

impl<
    T,
    S: RegistryStorage<Value = Box<T>>
> ManualRegistry<S> 
{

    /// Safety:
    /// Access cannot be used to 'acquire' a reference to Box<T>
    pub unsafe fn safer_replace<W, Access: Accessor<StoredValue = S::Value, Value = W>>(
        &mut self,
        manual_registry_replacement_input: ManualRegistryReplacementInput<'_, Access, S::Key, Access::Value>
    ) -> ManualRegistryReplacementResult<Access::StoredValue> 
    {
        let span = span!(FUNCTION_LEVEL, "Manual Safe Replacement");
        let _enter = span.enter();

        // Safety:
        // if a container of Box reallocates, pointers to the Box are still valid
        // note: references to the box itself can still be made invalid
        unsafe { self.replace(manual_registry_replacement_input) }
    }
}