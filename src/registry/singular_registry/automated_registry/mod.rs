use std::cell::UnsafeCell;

use tracing::span;

use crate::prelude::{Accessor, FUNCTION_LEVEL, ManualRegistry, ManualRegistryAccessInput, ManualRegistryAccessResult, ManualRegistryReplacementInput, ManualRegistryReplacementResult, RegistryStorage};

pub mod manual_registry;

pub struct AutomatedRegistry<S> {
    manual_registry: UnsafeCell<ManualRegistry<S>>
}

impl<S: RegistryStorage> AutomatedRegistry<S> {
    unsafe fn get_inner(&self) -> &ManualRegistry<S> {
        unsafe { & *self.manual_registry.get() }
    }

    unsafe fn get_inner_mut(&self) -> &mut ManualRegistry<S> {
        unsafe { &mut *self.manual_registry.get() }
    }

    pub unsafe fn access<Access: Accessor<StoredValue = S::Value>>(
        &self,
        input: ManualRegistryAccessInput<'_, Access, S::Key>
    ) -> ManualRegistryAccessResult<Access::AccessResult<'_, Access::Value>> {
        let span = span!(FUNCTION_LEVEL, "Automated Access");
        let _enter = span.enter();

        unsafe { self.get_inner() }.access(input)
    }

    pub unsafe fn replace<Access: Accessor<StoredValue = S::Value>>(
        &self,
        input: ManualRegistryReplacementInput<'_, Access, S::Key, Access::Value>
    ) -> ManualRegistryReplacementResult<Access::AccessResult<'_, Access::StoredValue>> {
        let span = span!(FUNCTION_LEVEL, "Automated Replacement");
        let _enter = span.enter();

        unsafe { self.get_inner_mut() }.replace(input)
    }

    pub fn contains_key(
        &self,
        key: &S::Key
    ) -> bool {
        unsafe { self.get_inner() }.contains_key(key)
    }
}