use tracing::span;

use crate::prelude::{Accessor, FUNCTION_LEVEL, ManualRegistryAccessInput, ManualRegistryAccessResult, ManualRegistryReplacementInput, ManualRegistryReplacementResult, Storage};

pub mod storage;
pub mod manual_registry_input;
pub mod manual_registry_result;

pub struct ManualRegistry<S> {
    storage: S,
}

impl<
    S: Storage,
> ManualRegistry<S> {
    pub fn access<
        Access: Accessor<StoredValue = S::Value>
    >(
        &self, 
        input: ManualRegistryAccessInput<'_, Access, S::Key>
    ) -> ManualRegistryAccessResult<Access::AccessResult<'_, Access::Value>> {
        let span = span!(FUNCTION_LEVEL, "Manual Access");
        let _enter = span.enter();

        match self.storage.get(input.key) {
            Some(value) => ManualRegistryAccessResult::Found(input.access.access(value)),
            None => ManualRegistryAccessResult::NotFound,
        }
    }

    pub fn replace<
        Access: Accessor<StoredValue = S::Value>
    >(
        &mut self,
        input: ManualRegistryReplacementInput<'_, Access, S::Key, Access::Value>
    ) -> ManualRegistryReplacementResult<Access::AccessResult<'_, Access::StoredValue>> {
        let span = span!(FUNCTION_LEVEL, "Manual Replacement");
        let _enter = span.enter();

        let ManualRegistryReplacementInput {
            access,
            key,
            value
        } = input;

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