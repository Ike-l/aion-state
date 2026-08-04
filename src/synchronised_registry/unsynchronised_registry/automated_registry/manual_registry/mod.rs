use stable_deref_trait::StableDeref;
use tracing::{Level, event};

use crate::prelude::{Accessor, AccessorResult, ManualRegistryAccessError, ManualRegistryAccessInput, ManualRegistryReplacementInput, ManualRegistryReplacementResult, RegistryStorage, StoredValueTrait, trace_function};

pub mod registry_storage;
pub mod manual_registry_input;
pub mod manual_registry_result;

/// Wraps storage using `Accessor`
#[derive(Default)]
pub struct ManualRegistry<S> {
    storage: S,
}

impl<
    S: RegistryStorage,
> ManualRegistry<S> 
{
    pub fn acquire_access<'a, Access: Accessor, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(
        &'a mut self, 
        ManualRegistryAccessInput {
            value_id, access
        }: ManualRegistryAccessInput<'_, S::ValueId, Access>
    ) -> Result<AccessResult, ManualRegistryAccessError> 
        where <S as RegistryStorage>::Value: StoredValueTrait
    {
        trace_function!("Manual Acquire Access");

        match self.storage.get_mut(value_id) {
            Some(stored_value) => {
                Ok(access.acquire::<S::Value, AccessResult>(stored_value))
            },
            None => Err(ManualRegistryAccessError::NotFound),
        }
    }

    // private until i can figure out a way to detect reallocations before they happen
    /// Safety:
    /// Insert won't invalidate concurrent access
    /// ^ i.e do not "replace" a borrowed item
    /// ^ i.e do not insert if it could reallocate container and invalidate concurrent accesses
    unsafe fn replace<Access: Accessor>(
        &mut self,
        ManualRegistryReplacementInput {
            access, value_id, value
        }: ManualRegistryReplacementInput<'_, Access, S::ValueId, <S::Value as StoredValueTrait>::Value>
    ) -> ManualRegistryReplacementResult<<S::Value as StoredValueTrait>::Value>
        where <S as RegistryStorage>::Value: StoredValueTrait
    {
        trace_function!("Manual Unsafe Replacement");

        let old_resource = match (
            value,
            self.storage.contains_key(&value_id),
            access.can_insert_resource(),
            access.can_remove_resource(),
        ) {
            (_, true, _, false) => {
                event!(Level::WARN, "Access Cannot Remove Stored Value");
                return ManualRegistryReplacementResult::DeniedAccess
            },
            // if input contains resource (so insert) but denied insert
            (Some(_), _, false, _) => {
                event!(Level::WARN, "Access Cannot Insert Given Value");
                return ManualRegistryReplacementResult::DeniedAccess
            },

            // if does not contain resource and not input 
            (None, false, _, _) => return ManualRegistryReplacementResult::NoOp,

            // removal and allowed remove
            (None, true, _, true) => self.storage.remove(&value_id),
            
            // replacement and allowed insert & remove
            (Some(new_value), true, true, true) => {
                event!(Level::DEBUG, "Access Can Replace");
                self.storage.insert(value_id, S::Value::new(new_value))
            },

            // insert without replacement and allowed insert
            (Some(new_value), false, true, _) => {
                event!(Level::DEBUG, "Access Can Insert");
                self.storage.insert(value_id, S::Value::new(new_value))
            },            
        };

        match old_resource {
            Some(found) => ManualRegistryReplacementResult::Found(found.into_inner()),
            None => ManualRegistryReplacementResult::NotFound,
        }
    }

    // safer bc trade reallocation requirement for the don't reference the stable address stored value itself
    /// # Safety
    /// 
    /// Access cannot be used to 'acquire' a reference to StableAddress itself
    /// 
    /// Insert won't invalidate concurrent access
    /// 
    /// ^ i.e do not replace a borrowed item
    pub unsafe fn reallocating_replace<Access: Accessor>(
        &mut self,
        manual_registry_replacement_input: ManualRegistryReplacementInput<'_, Access, S::ValueId, <S::Value as StoredValueTrait>::Value>
    ) -> ManualRegistryReplacementResult<<S::Value as StoredValueTrait>::Value> 
        where <S as RegistryStorage>::Value: StableDeref + StoredValueTrait
    {
        trace_function!("Manual Safe Replacement");

        // Safety:
        // if a container of StableAddress reallocates, pointers are still valid
        unsafe { self.replace(manual_registry_replacement_input) }
    }

    pub fn contains_key(
        &self,
        key: &S::ValueId
    ) -> bool {
        trace_function!("Manual Contains Key");

        self.storage.contains_key(key)
    }

    pub fn len(&self) -> usize {
        trace_function!("Manual Registry Len");

        self.storage.len()
    }
}

impl<
    S: RegistryStorage,
> ManualRegistry<S> 
    where S::ValueId: Clone
{
    pub fn keys(&self) -> Vec<S::ValueId> {
        trace_function!("Manual Registry keys");

        self.storage.keys().cloned().collect()
    }
}
