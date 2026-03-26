use std::cell::UnsafeCell;

use crate::prelude::{Accessor, ManualRegistry, ManualRegistryAccessInput, ManualRegistryAccessResult, ManualRegistryCheckAccess, ManualRegistryCheckAccessResult, ManualRegistryRelease, ManualRegistryReleaseResult, ManualRegistryReplacementInput, ManualRegistryReplacementResult, RegistryStorage, StableAddress, trace_function};

pub mod manual_registry;

/// Provides individual access ability to the underlying storage
/// 
/// Requires `Automation` to ensure safety
pub struct AutomatedRegistry<S> {
    manual_registry: UnsafeCell<ManualRegistry<S>>
}

impl<S: RegistryStorage> AutomatedRegistry<S> {
    /// Safety: No Concurrent Unique References
    unsafe fn get_inner(&self) -> &ManualRegistry<S> {
        unsafe { & *self.manual_registry.get() }
    }

    /// Safety: No Concurrent References
    unsafe fn get_inner_mut(&self) -> &mut ManualRegistry<S> {
        unsafe { &mut *self.manual_registry.get() }
    }

    /// Safety: No Concurrent Unique References
    pub unsafe fn acquire_access<Access: Accessor<StoredValue = S::Value>>(
        &self,
        input: ManualRegistryAccessInput<'_, S::ValueId, Access>
    ) -> ManualRegistryAccessResult<Access::AccessResult<'_>> {
        trace_function!("Automated Registry Acquire Access");

        unsafe { self.get_inner() }.acquire_access(input)
    }

    /// Safety: 
    /// No Concurrent References
    /// Access cannot be used to 'acquire' a reference to StableAddress itself
    /// Insert won't invalidate concurrent access
    /// ^ i.e do not replace a borrowed item
    pub unsafe fn safer_replace<Access: Accessor<StoredValue = S::Value>>(
        &mut self,
        manual_registry_replacement_input: ManualRegistryReplacementInput<'_, Access, S::ValueId, Access::Value>
    ) -> ManualRegistryReplacementResult<Access::StoredValue> 
        where Access::StoredValue: StableAddress
    {
        trace_function!("Automated Safer Replacement");

        unsafe { self.get_inner_mut().safer_replace(manual_registry_replacement_input) }
    }

    pub fn check_access<Access: Accessor<StoredValue = S::Value>>(
        &self,
        input: &ManualRegistryCheckAccess<'_, S::ValueId, Access>
    ) -> ManualRegistryCheckAccessResult {
        trace_function!("Automated Registry Check Access");

        unsafe { self.get_inner().check_access(input) }
    }

    pub fn release<Access: Accessor<StoredValue = S::Value>>(
        &mut self,
        input: &ManualRegistryRelease<'_, S::ValueId, Access> 
    ) -> ManualRegistryReleaseResult {
        trace_function!("Automated Registry Release");

        unsafe { self.get_inner_mut().release(input) }
    }

    pub fn contains_key(
        &self,
        key: &S::ValueId
    ) -> bool {
        trace_function!("Automated Registry Contains Key");

        unsafe { self.get_inner() }.contains_key(key)
    }
}